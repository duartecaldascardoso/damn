damn() {
    if [[ $# -eq 0 ]]; then
        local last_cmd
        last_cmd=$(fc -ln -1)
        last_cmd="${last_cmd##*( )}"
        last_cmd="${last_cmd%%*( )}"
        if [[ -z "$last_cmd" ]] || [[ "$last_cmd" == damn* ]]; then
            echo "damn: No previous command found to correct."
            return 1
        fi
        local suggestion
        suggestion=$(damn-bin suggest "$last_cmd")
        local ret=$?
        if [[ $ret -eq 0 && -n "$suggestion" ]]; then
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
    [[ $- == *i* ]] || return

    local last_cmd="$BASH_COMMAND"
    last_cmd="${last_cmd#"${last_cmd%%[![:space:]]*}"}"
    last_cmd="${last_cmd%"${last_cmd##*[![:space:]]}"}"

    if [ -n "$last_cmd" ] && \
       [[ "$last_cmd" != damn* ]] && \
       [[ "$last_cmd" != "clear" ]] && \
       [[ "$last_cmd" != "list" ]] && \
       [[ "$last_cmd" != "history" ]]; then
        echo "$last_cmd" >> ~/.damn_history
    fi
}

trap 'damn_log_command' DEBUG