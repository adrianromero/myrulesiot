#!/bin/sh

while :
do
    cargo run

    file_path="engine_exit"
    if [ -f "$file_path" ]; then
        file_contents=$(cat "$file_path")
        if [ "$file_contents" = "NORMAL/reboot" ]; then
            # Just reboot
            echo "Rebooting..."
        elif  [ "$file_contents" = "NORMAL/upgrade" ]; then
            # Upgrade then reboot
            echo "Upgrading..."
            pull origin release
        else 
            echo "Finishing..."
            break
        fi
    else
        # Abnormal exit
        break
    fi
done
