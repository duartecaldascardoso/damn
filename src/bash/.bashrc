damn() {
    if [ "$#" -eq 0 ]; then
        local last_cmd=""
        # Look back through the last 20 commands for the first non-damn command
        for i in {2..20}; do
            last_cmd=$(history $i | head -n1 | sed 's/^[ ]*[0-9]\+[ ]*//')
            if [ -n "$last_cmd" ] && [[ "$last_cmd" != damn* ]]; then
                break
            fi
        done

        if [ -z "$last_cmd" ]; then
            echo "damn: No previous command found to correct."
            return 1
        fi

        local suggestion
        suggestion=$(damn-bin suggest "$last_cmd")
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

export PROMPT_COMMAND='
  if [ $? -eq 0 ]; then
    last_cmd=$(history 1 | sed "s/^[ ]*[0-9]\+[ ]*//")
    if [ -n "$last_cmd" ] && \
       [[ "$last_cmd" != damn* ]] && \
       [[ "$last_cmd" != "clear" ]] && \
       [[ "$last_cmd" != "history" ]]; then
      echo "$last_cmd" >> ~/.damn_history
    fi
  fi
'