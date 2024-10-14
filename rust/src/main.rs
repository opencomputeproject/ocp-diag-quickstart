use std::env;
use std::io::Write;
use std::time::Duration;

use anyhow::Result;
use num_bigint::{BigUint, ToBigUint};
use serde::Serialize;
use thiserror::Error;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tokio::sync::oneshot::{self, error::TryRecvError};

use ocptv::output::{self as tv, DiagnosisType, OcptvError, TestResult, TestStatus};
use ocptv::{ocptv_diagnosis_pass, ocptv_error, ocptv_log_error, ocptv_log_info};

const STRESSOR_BIN: &str = "";

fn compute(n: BigUint) -> BigUint {
    n * 2.to_biguint().unwrap()
}

#[allow(dead_code)]
#[derive(Error, Debug)]
enum SystemError {
    #[error("cannot get system state")]
    Fail,
}

impl From<SystemError> for OcptvError {
    fn from(value: SystemError) -> Self {
        OcptvError::Other(Box::new(value))
    }
}

struct SystemState {
    temperature: u32,
}

/// Measure system state. This is a mock.
/// In your real diagnostic you will get the system temperature (and possibly
/// other system state) for the component of interest
fn get_system_state() -> Result<SystemState, SystemError> {
    Ok(SystemState { temperature: 25 })
}

/// Stress the system with the binary provided.
async fn stress_system_with_binary(step: tv::ScopedTestStep) -> Result<TestStatus, tv::OcptvError> {
    let state = get_system_state()?;
    step.add_measurement_detail(
        tv::Measurement::builder("initial temperature", state.temperature)
            .unit("C")
            .build(),
    )
    .await?;

    if state.temperature > 30 {
        ocptv_error!(
            step,
            "initial temperature too high to proceed with the test"
        )
        .await?;
        step.add_diagnosis("high-initial-temperature", DiagnosisType::Fail)
            .await?;
    } else {
        ocptv_log_info!(step, "temperature is within limits").await?;
    }

    if STRESSOR_BIN == "" {
        return Ok(TestStatus::Skip);
    } else {
        // use stressor binary to stress the system
    }

    ocptv_diagnosis_pass!(step, "binary-pass").await?; // macro style with source location
    Ok(TestStatus::Complete)
}

/// Stress the system without external dependencies.
async fn stress_system_without_binary(
    step: tv::ScopedTestStep,
) -> Result<TestStatus, tv::OcptvError> {
    let temp = step.add_measurement_series_detail(
        tv::MeasurementSeriesDetail::builder("temperature")
            .unit("C")
            .build(),
    );

    let (tx, mut rx) = oneshot::channel::<u32>();

    let stressor = tokio::spawn(async move {
        let mut n = 2.to_biguint().unwrap();
        loop {
            match rx.try_recv() {
                Err(TryRecvError::Closed) => break,
                _ => {}
            }

            n = compute(n);
        }

        n
    });

    temp.scope(|s| async move {
        for _ in 0..5 {
            // Output a temperature measurement in the series every second
            tokio::time::sleep(Duration::from_secs(1)).await;

            let state = get_system_state()?;
            s.add_measurement(state.temperature).await?;
        }

        drop(tx);
        Ok(())
    })
    .await?;

    let filename = "stressor_output.txt";
    let path = env::current_dir()?.join(filename);

    let last = stressor.await.map_err(|e| OcptvError::Other(Box::new(e)))?;
    match fs::File::create(&path).await {
        Ok(mut file) => {
            let mut buf: Vec<u8> = vec![];
            writeln!(buf, "Calculated n:")?;
            writeln!(buf, "{}", last.to_string())?;
            file.write_all(&buf).await?;

            step.add_file(filename, tv::Uri::from_file_path(&path).unwrap())
                .await?;
        }
        Err(e) => {
            ocptv_log_error!(
                step,
                &format!("failed to write output file: {}", e.to_string())
            )
            .await?;
        }
    }

    #[derive(Serialize)]
    struct Extension {
        #[serde(rename = "type")]
        ext_type: String,
    }

    // Add some custom structured data relevant to your company
    step.add_extension(
        "mycompany-test_plan",
        Extension {
            ext_type: "test_42".to_owned(),
        },
    )
    .await?;

    step.add_diagnosis("no-binary-pass", DiagnosisType::Pass)
        .await?;
    Ok(TestStatus::Complete)
}

#[tokio::main]
async fn main() -> Result<()> {
    let run = tv::TestRun::builder("hello_world", "1.0").build();
    let dut = tv::DutInfo::builder("dut0")
        .name("host0.example.com")
        .build();

    run.scope(dut, |run| async move {
        if let Err(e) = get_system_state() {
            ocptv_error!(
                run,
                "no-temperature",
                &format!("system state cannot be retrieved: {}", e)
            )
            .await?;

            return Ok(tv::TestRunOutcome {
                status: TestStatus::Skip,
                result: TestResult::NotApplicable,
            });
        }

        ocptv_log_info!(run, "We are starting the Hello World diagnostic").await?;

        run.add_step("run binary")
            .scope(stress_system_with_binary)
            .await?;

        run.add_step("stress system without an external binary")
            .scope(stress_system_without_binary)
            .await?;

        Ok(tv::TestRunOutcome {
            status: TestStatus::Complete,
            result: TestResult::Pass,
        })
    })
    .await?;

    Ok(())
}
