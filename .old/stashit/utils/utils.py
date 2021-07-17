import datetime
import subprocess
import os


def get_date() -> str:
    return datetime.datetime.now().strftime("%Y-%m-%d")


def debug(job: str) -> None:
    """
    Stupid debug messages to make the script look fun

    """
    if job == "createdir":
        return f"job: creating new folder %s"
    if job == "error":
        return f"{job}: cannot make folder, already exists"


def gen_files(amount=5) -> None:
    """
    Generate files for testing purposes

    """
    for i in range(amount):
        subprocess.run(["touch", f"stubfile-{i}"])


def create_folder(name="stashit-" + get_date()) -> None:
    """
    This function was put in utils to minimize the eyesore created
    by having to write a subprocess method each time.
    """
    os.mkdir(f"{name}")
