FROM fedora:42

ARG USERNAME
ARG USER_UID=995
ARG USER_GID=$USER_UID
ARG ARCH
ENV TZ="America/New_York"

# install pre-reqs
RUN dnf update -y \
    && dnf -y install dnf-plugins-core ca-certificates curl wget net-tools git sudo tree which \
    ripgrep fd-find unzip vim tmux make gcc patch zlib-devel bzip2 bzip2-devel readline-devel \
    sqlite sqlite-devel openssl-devel tk-devel libffi-devel xz-devel libuuid-devel gdbm-libs \
    libnsl2 awk \
    && update-ca-trust extract \
    && dnf -y clean all

# Add a non-root user and enable them to use sudo without password
RUN groupadd --gid $USER_GID $USERNAME \
    && useradd -s /bin/bash --uid $USER_UID --gid $USER_GID -m $USERNAME \
    && sed -i /etc/sudoers -re 's/^%sudo.*/%sudo ALL=(ALL:ALL) NOPASSWD: ALL/g' \
    && sed -i /etc/sudoers -re 's/^root.*/root ALL=(ALL:ALL) NOPASSWD: ALL/g' \
    && sed -i /etc/sudoers -re 's/^#includedir.*/## **Removed the include directive** ##"/g' \
    && echo "$USERNAME ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers \
    && echo "Customized the sudoers file for passwordless access to the $USERNAME user!" \
    && echo "$USERNAME user:";  su - $USERNAME -c id

USER $USERNAME
WORKDIR /home/$USERNAME/

# Option 1: Accessing the Host's Docker Daemon (Recommended for most dev containers)
# RUN sudo dnf config-manager addrepo --from-repofile=https://download.docker.com/linux/fedora/docker-ce.repo \
#     && sudo dnf install -y docker-ce-cli docker-buildx-plugin docker-compose-plugin \
#     && sudo usermod -aG docker $USERNAME


# Option 2: Docker-in-Docker (DinD) - Running Docker Daemon Inside the Container
# RUN sudo dnf config-manager addrepo --from-repofile=https://download.docker.com/linux/fedora/docker-ce.repo \
#     && sudo dnf install -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin socat \
#     && sudo usermod -aG docker $USERNAME \
#     && echo -e "#!/bin/sh\n\
# sudoIf() { if [ \"\$(id -u)\" -ne 0 ]; then sudo \"\$@\"; else \"\$@\"; fi }\n\
# sudoIf rm -rf /var/run/docker.sock\n\
# ((sudoIf socat UNIX-LISTEN:/var/run/docker.sock,fork,mode=660,user=${USERNAME} UNIX-CONNECT:/var/run/docker-host.sock) 2>&1 >> /tmp/vscr-docker-from-docker.log) & > /dev/null\n\
# \"\$@\"" | sudo tee -a /usr/local/share/docker-init.sh \
#     && sudo chmod +x /usr/local/share/docker-init.sh \
#     && sudo usermod -aG docker $USERNAME

# install rust
ENV CARGO_HOME="/home/$USERNAME/.cargo"
ENV RUSTUP_HOME="/home/$USERNAME/.rustup"
ENV PATH="$CARGO_HOME/bin:$RUSTUP_HOME:$PATH"

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -sSf | sh -s -- --verbose -y --profile default \
    && rustup component add rust-analysis \
    && rustup component add rust-src \
    && rustup component add rust-analyzer \
    && rustup target add x86_64-unknown-linux-gnu \
    && rustup target add x86_64-unknown-linux-musl \
    && rustup target add x86_64-pc-windows-msvc \
    && rustup target add aarch64-unknown-linux-gnu \
    && rustup target add aarch64-pc-windows-msvc \
    && rustup target add aarch64-apple-darwin \
    && rustup target add wasm32-wasip1

    # install starship and terminal configuration
# RUN $CARGO_HOME/bin/cargo install starship --locked \
#     && echo "eval '$(starship init bash)'" >> /home/$USERNAME/.bashrc

# install asdf and required plugins   #see  https://github.com/asdf-vm/asdf/releases
RUN curl -Lo asdf.tar.gz "https://github.com/asdf-vm/asdf/releases/download/v0.18.0/asdf-v0.18.0-linux-arm64.tar.gz" \
    && sudo tar -xvzf asdf.tar.gz -C /usr/local/bin/

RUN asdf plugin add starship && asdf install starship latest && asdf set -u starship latest \
    && asdf plugin add eza && asdf install eza latest && asdf set -u eza latest

COPY ./resources/.bashrc /home/$USERNAME/
COPY ./resources/starship.toml /home/$USERNAME/.config/


# Setting the ENTRYPOINT to docker-init.sh will configure non-root access
# to the Docker socket. The script will also execute CMD as needed.
# ENTRYPOINT [ "/usr/local/share/docker-init.sh" ]

# Option 2: Docker-in-Docker (DinD)
# ENTRYPOINT ["/bin/sh", "-c", "\
#     sudo dockerd --host=unix:///var/run/docker-host.sock & \
#     until [ -S /var/run/docker-host.sock ]; do echo 'Waiting for dockerd socket...'; sleep 1; done; \
#     sudo -E -u \"$USERNAME\" -- /usr/local/share/docker-init.sh bash \
# "]

# ENTRYPOINT [ "/bin/bash" ] 
# CMD [ "sleep", "infinity" ]
