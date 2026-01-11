#!/bin/sh

set -e
 
export opt="" VERBOSE=false PREFIX="${PWD}"

print_help() {
    echo "Usage: $0 <subcommand> [options]"
    echo "Available subcommands: build, clean"
    echo "Options:"
    echo "  -h            Show this help message"
    echo "  -p <prefix>   Set the prefix directory (default: current directory)"
    echo "  -v            Enable verbose output"
    echo ""
    echo "rusttale documentation, the lore build scripts - lazypwny751, 2026"
}

if [ "${#}" -le 0 ]; then
    echo "You need to provide at least one argument"
    exit 1
fi

export SUBCMD="${1}" && shift

while getopts ":hp:v" opt; do
    case "${opt}" in
        "v")
            export VERBOSE=true
        ;;
        "p")
            export PREFIX="${OPTARG}"
        ;;
        "h")
            print_help
            exit 0
        ;;
        \?)
            echo "Invalid option: -${OPTARG}" >&2
            exit 1
        ;;
    esac
done

shift $((OPTIND - 1))

# Defaulting, set defaults to Turkish and English translation
if [ "${#}" -ne 1 ]; then
    set -- "tr" "en"
fi

case "${SUBCMD}" in
    "help"|"--help"|"-h")
        print_help
        exit 0
    ;;
    "build")
        if [ ! -e "${PREFIX}/build" ]; then
            mkdir -p "${PREFIX}/build"
        fi
        for dir in "$@"; do
            echo "Building documentation for locale: ${dir##*/}"
        done
    ;;
    "clean")
        echo "Cleaning build directory at 'build'"
    ;;
    *)
        echo "Unknown subcommand: ${SUBCMD}"
        echo "Use '${0##*/} --help' to see available subcommands."
        exit 1
    ;;
esac