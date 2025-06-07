damn() {
    if [ "$#" -eq 0 ]; then
        local last_cmd=""
        # Use command substitution to avoid subshell
        for line in $(history 20 | tail -r); do
            cmd=$(echo "$line" | sed 's/^[ ]*[0-9]\+[ ]*//')
            if [ -n "$cmd" ] && [[ "$cmd" != damn* ]]; then
                last_cmd="$cmd"
                break
            fi
        done

        if [ -z "$last_cmd" ]; then
            echo "damn: No previous command found to correct."
            return 1
        fi

        local suggestion
        suggestion=$(damn-bin suggest "$last_cmd" 2>&1)
        local ret=$?

        if [ $ret -eq 0 ] && [ -n "$suggestion" ]; then
            echo "damn: Running -> $suggestion"
            eval "$suggestion"
        else
            echo "damn: No suggestion found."
            return 1
        fi
    else
        damn-bin "$@"
    fi
}

damn_log_command() {
    # Only log interactive shell commands
    [[ $- == *i* ]] || return

    # $BASH_COMMAND is the command about to be executed
    local last_cmd="$BASH_COMMAND"
    # Remove leading/trailing spaces
    last_cmd="${last_cmd#"${last_cmd%%[![:space:]]*}"}"
    last_cmd="${last_cmd%"${last_cmd##*[![:space:]]}"}"

    # Filter out damn and its subcommands
    if [ -n "$last_cmd" ] && \
       [[ "$last_cmd" != damn* ]] && \
       [[ "$last_cmd" != "clear" ]] && \
       [[ "$last_cmd" != "list" ]] && \
       [[ "$last_cmd" != "history" ]]; then
        echo "$last_cmd" >> ~/.damn_history
    fi
}

# Set the DEBUG trap
trap 'damn_log_command' DEBUG