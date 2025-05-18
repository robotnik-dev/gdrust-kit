#!/usr/bin/env python3
"""
Script to update versions in the gdrust_kit workspace.
This dynamically discovers crates in the crates/ directory and allows
updating versions independently or synchronizing them as needed.
It also updates version references in README.md files.
"""

import re
import argparse
from pathlib import Path


def parse_args():
    parser = argparse.ArgumentParser(description="Update crate versions in the workspace", usage='%(prog)s [option] [version]')
    parser.add_argument("-m", "--main", metavar='[version]', help="Update main package version", required=False)
    parser.add_argument("-s", "--sync", metavar='[version]', help="Synchronize all to version", required=False)
    parser.add_argument("-l", "--list", action="store_true", help="List all available crates", required=False)

    workspace_root = Path(__file__).parent.parent
    crates_dir = workspace_root / "crates"

    if crates_dir.exists() and crates_dir.is_dir():
        for crate_dir in crates_dir.iterdir():
            if crate_dir.is_dir() and (crate_dir / "Cargo.toml").exists():
                crate_name = crate_dir.name
                parser.add_argument(
                    f"--{crate_name.replace('_', '-').replace('gdrust-', '')}",
                    metavar='[version]',
                    help=f"Update {crate_name} version",
                    required=False,
                )
    return parser.parse_args()


def update_toml_version(file_path, new_version):
    """Update the version in a Cargo.toml file."""
    with open(file_path, 'r') as f:
        content = f.read()

    # Replace version in package section
    pattern = r'(^\[package\].*?version\s*=\s*)"([^"]+)"'
    repl = f'\\1"{new_version}"'
    updated = re.sub(pattern, repl, content, flags=re.DOTALL | re.MULTILINE)

    with open(file_path, 'w') as f:
        f.write(updated)

    print(f"Updated {file_path} to version {new_version}")


def update_readme_version(workspace_root, crate_name, new_version):
    """Update version references in README.md files."""
    # Check for README in the crate directory
    readme_paths = []

    if crate_name == "main":
        # Main crate README is in the workspace root
        readme_paths.append(workspace_root / "README.md")
    else:
        # Check for README in crate directory
        crate_dir = workspace_root / "crates" / crate_name
        if crate_dir.exists():
            readme_paths.append(crate_dir / "README.md")

    for readme_path in readme_paths:
        if not readme_path.exists():
            continue

        with open(readme_path, 'r') as f:
            content = f.read()

        # Update version references in dependency examples
        # Match various forms of version specifications in code blocks
        patterns = [
            (r'(version\s*=\s*")([^"]+)(")', lambda m: f'{m.group(1)}{new_version}{m.group(3)}'),
            (r'(\{\s*version\s*=\s*")([^"]+)(")', lambda m: f'{m.group(1)}{new_version}{m.group(3)}'),
            (r'(\w+\s*=\s*\{\s*version\s*=\s*")([^"]+)(")', lambda m: f'{m.group(1)}{new_version}{m.group(3)}'),
        ]

        updated_content = content
        for pattern, replacement in patterns:
            updated_content = re.sub(pattern, replacement, updated_content)

        if updated_content != content:
            with open(readme_path, 'w') as f:
                f.write(updated_content)
            print(f"Updated version references in {readme_path} to {new_version}")



def update_dependency_version(file_path, dependency_name, new_version):
    """Update a specific dependency version in a Cargo.toml file."""
    with open(file_path, 'r') as f:
        content = f.read()

    # Pattern for dependency in different formats
    patterns = [
        # Regular dependency
        rf'(^{dependency_name}\s*=\s*{{.*?version\s*=\s*)"([^"]+)"',
        # Inline dependency
        rf'(^{dependency_name}\s*=\s*)"([^"]+)"',
    ]

    updated = content
    for pattern in patterns:
        updated = re.sub(pattern, f'\\1"{new_version}"', updated, flags=re.DOTALL | re.MULTILINE)

    if updated != content:
        with open(file_path, 'w') as f:
            f.write(updated)
        print(f"Updated dependency {dependency_name} in {file_path} to version {new_version}")


def find_crates(workspace_root):
    """Find all crates in the workspace."""
    crates = {}

    # Main crate
    main_toml = workspace_root / "Cargo.toml"
    if main_toml.exists():
        crates["main"] = main_toml

    # Sub-crates in crates directory
    crates_dir = workspace_root / "crates"
    if crates_dir.exists() and crates_dir.is_dir():
        for crate_dir in crates_dir.iterdir():
            if crate_dir.is_dir():
                cargo_toml = crate_dir / "Cargo.toml"
                if cargo_toml.exists():
                    crates[crate_dir.name] = cargo_toml

    return crates


def find_crate_dependencies(crates):
    """Find dependencies between crates."""
    dependencies = {}

    for crate_name, toml_path in crates.items():
        with open(toml_path, 'r') as f:
            content = f.read()

        dependencies[crate_name] = []

        # Check for dependencies on other crates in the workspace
        for dep_name in crates.keys():
            if dep_name == crate_name:
                continue

            # Different patterns for dependencies
            patterns = [
                rf'^{dep_name}\s*=\s*{{.*?version\s*=\s*"([^"]+)"',  # Regular dependency
                rf'^{dep_name}\s*=\s*"([^"]+)"',  # Inline dependency
            ]

            for pattern in patterns:
                match = re.search(pattern, content, flags=re.MULTILINE | re.DOTALL)
                if match:
                    dependencies[crate_name].append(dep_name)
                    break

    return dependencies


def main():
    args = parse_args()

    # Get the workspace root (script is in scripts/ directory)
    workspace_root = Path(__file__).parent.parent

    # Find all crates in the workspace
    crates = find_crates(workspace_root)

    # List available crates if requested
    if args.list:
        for crate_name in crates.keys():
            print(f"  {crate_name}")
        return

    # Find dependencies between crates
    dependencies = find_crate_dependencies(crates)

    # If --sync is used, update all versions to the same value
    if args.sync:
        for crate_name, toml_path in crates.items():
            update_toml_version(toml_path, args.sync)
            # Update README for this crate
            update_readme_version(workspace_root, crate_name, args.sync)

        # Also update dependency references
        for crate_name, deps in dependencies.items():
            for dep_name in deps:
                update_dependency_version(crates[crate_name], dep_name, args.sync)

        return

    # Otherwise, update specified crates individually
    updated_crates = {}

    # Update main crate if specified
    if args.main and "main" in crates:
        update_toml_version(crates["main"], args.main)
        update_readme_version(workspace_root, "main", args.main)
        updated_crates["main"] = args.main

    # Process arguments for each crate
    for crate_name, toml_path in crates.items():
        if crate_name == "main":
            continue

        # Convert crate_name to arg format (replace underscores with hyphens)
        arg_name = crate_name.replace("_", "-")
        if hasattr(args, arg_name) and getattr(args, arg_name) is not None:
            new_version = getattr(args, arg_name)
            update_toml_version(toml_path, new_version)
            update_readme_version(workspace_root, crate_name, new_version)
            updated_crates[crate_name] = new_version

    # Update dependencies for crates that were updated
    for crate_name, deps in dependencies.items():
        for dep_name in deps:
            if dep_name in updated_crates:
                update_dependency_version(crates[crate_name], dep_name, updated_crates[dep_name])


if __name__ == "__main__":
    main()
