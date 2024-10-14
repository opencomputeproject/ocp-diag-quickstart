import os
import pathlib
import socket
import sys
import time

import ocptv.output as tv

# Path to binary that will be used to stress the system
stressor_binary: str = ""


def compute(n: int) -> int:
    """
    Do some computation to stress the system
    """
    return 2 * n


def stress_system_without_binary(step: tv.TestStep) -> None:
    """
    Stress the system without external dependencies.
    """
    start_time = int(time.time())
    previous_elapsed_seconds = -1
    n = 2
    temp = step.start_measurement_series(name="temperature", unit="C")
    with temp.scope():
        while (elapsed_seconds := int(time.time())) < start_time + 5:
            n = compute(n)
            # Output a temperature measurement in the series every second
            if elapsed_seconds > previous_elapsed_seconds:
                temp.add_measurement(value=get_system_state()["temperature"])
                previous_elapsed_seconds = elapsed_seconds

    output_directory = os.path.dirname(os.path.realpath(__file__))
    output_file = "stressor_output.txt"
    output_path = os.path.join(output_directory, output_file)
    try:
        with open(output_path, "w") as f:
            if hasattr(sys, "set_int_max_str_digits"):
                sys.set_int_max_str_digits(0)
            f.write(f"Calculated n:\n{n}\n")
    except Exception as e:
        step.add_log(
            severity=tv.LogSeverity.ERROR,
            message=f"Failed to write output file: {e}",
        )
    else:
        step.add_file(name=output_file, uri=pathlib.Path(output_path).as_uri())

    # Add some custom structured data relevant to your company
    step.add_extension(
        name="mycompany-master_test_plan",
        content="test_42",
    )

    step.add_diagnosis(diagnosis_type=tv.DiagnosisType.PASS, verdict="no-binary-pass")


def stress_system_with_binary(step: tv.TestStep) -> None:
    """
    Stress the system with the binary provided.
    """
    system_state = get_system_state()
    step.add_measurement(
        name="Initial temperature", value=system_state["temperature"], unit="C"
    )
    if system_state["temperature"] > 30:
        step.add_error(
            symptom="high-temperature",
            message="Initial temperature is too high to proceed with test",
        )
        step.add_diagnosis(
            diagnosis_type=tv.DiagnosisType.FAIL, verdict="high-initial-temperature"
        )
        return
    else:
        step.add_log(
            severity=tv.LogSeverity.INFO, message="Temperature is within limits"
        )

    if stressor_binary == "":
        raise tv.TestStepError(status=tv.TestStatus.SKIP)
    else:
        # Use stressor binary to stress the system
        pass
    step.add_diagnosis(diagnosis_type=tv.DiagnosisType.PASS, verdict="binary-pass")


def get_system_state() -> dict:
    """
    Measure system state. This is a mock.
    In your real diagnostic you will get the system temperature (and possibly
    other system state) for the component of interest
    """
    return {
        "temperature": 25,
    }


def main():
    run = tv.TestRun(name="HelloWorld", version="1.0", parameters={})
    with run.scope(dut=tv.Dut(id=socket.gethostname())):
        if not get_system_state().get("temperature"):
            run.add_error(
                symptom="no-temperature", message="Temperature cannot be retrived"
            )
            raise tv.TestRunError(
                status=tv.TestStatus.SKIP, result=tv.TestResult.NOT_APPLICABLE
            )

        run.add_log(
            severity=tv.LogSeverity.INFO,
            message="We are starting the Hello World diagnostic",
        )

        stepA = run.add_step("Run binary")
        with stepA.scope():
            stress_system_with_binary(stepA)

        stepB = run.add_step("Stress system without an external binary")
        with stepB.scope():
            stress_system_without_binary(stepB)


if __name__ == "__main__":
    main()
