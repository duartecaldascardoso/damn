damn() {
    if [[ $# -eq 0 ]]; then
        # Find the most recent command that is NOT 'damn*'
        local last_cmd
        last_cmd=""
        # Look back through the last 20 commands for the first non-damn command
        for ((i=2; i<=20; i++)); do
            last_cmd=$(fc -ln -$i | head -n1)
            last_cmd="${last_cmd##*( )}"
            last_cmd="${last_cmd%%*( )}"
            if [[ -n "$last_cmd" ]] && [[ "$last_cmd" != damn* ]]; then
                break
            fi
        done

        if [[ -z "$last_cmd" ]]; then
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

precmd() {
    if [[ $? -eq 0 ]]; then
        local last_cmd
        last_cmd=$(fc -ln -1)
        last_cmd="${last_cmd##*( )}"
        last_cmd="${last_cmd%%*( )}"

        if [[ -n "$last_cmd" ]] && \
           [[ "$last_cmd" != damn* ]] && \
           [[ "$last_cmd" != "clear" ]] && \
           [[ "$last_cmd" != "history" ]]; then
            echo "$last_cmd" >> ~/.damn_history
        fi
    fi
}