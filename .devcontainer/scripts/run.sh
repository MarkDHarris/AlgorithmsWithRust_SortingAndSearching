#!/bin/bash
start_time=$(date +%s)

USERNAME="maharris"
IMAGENAME="rustdev:latest"

# docker run -it --rm --volume /var/run/docker.sock:/var/run/docker.sock \
# --mount "type=bind,src=./,target=/home/$USERNAME/project" \
# --entrypoint /bin/bash $IMAGENAME

docker run -it --rm \
    --privileged \
    --name my-dind-container \
    --mount "type=bind,src=./,target=/home/$USERNAME/project" \
    $IMAGENAME

end_time=$(date +%s)
duration=$((end_time - start_time))

hours=$((duration / 3600))
minutes=$(( (duration % 3600) / 60 ))
seconds=$((duration % 60))

printf "%02d:%02d:%02d" $hours $minutes $seconds