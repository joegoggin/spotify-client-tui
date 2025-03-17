printOpts() {
    clear
    echo "Press 'c' to clear screen"
    echo "Press 'q' to quit"
}

printOpts

tail -n +1 -f ~/.spotify-client-tui/logs/app.log &
PID=$!

while :; do
    read -rsn 1 key

    case "$key" in
    c)
        printOpts
        ;;
    q)
        kill $PID
        clear
        break
        ;;
    esac
done
