import os
from pathlib import Path

root_build_path = Path("debug/target")
osx_build_path = Path("osx")
osx_bundle_name = "Mango.app"


def build_osx():
    bundle_path = root_build_path / osx_build_path / osx_bundle_name
    if not bundle_path.exists():
        print("macOS bundle created")

    bundle_path.mkdir()


if __name__ == "__main__":
    build_osx()
