FROM fedora:42

ARG USERNAME
ARG USER_UID=1000
ARG USER_GID=$USER_UID
ARG ARCH
ENV TZ="America/New_York"

# install pre-reqs
RUN dnf update -y \
    && dnf -y install dnf-plugins-core ca-certificates curl wget iproute net-tools iptables git sudo tree which \
    shadow-utils passwd ripgrep fd-find unzip vim tmux make gcc patch zlib-devel bzip2 bzip2-devel readline-devel \
    sqlite sqlite-devel openssl-devel tk-devel libffi-devel xz-devel libuuid-devel gdbm-libs \
    procps libnsl2 awk \
    && update-ca-trust extract \
    && dnf -y clean all

# Add Docker CE repository
RUN curl -fsSL https://download.docker.com/linux/fedora/docker-ce.repo -o /tmp/docker-ce.repo && \
    mv /tmp/docker-ce.repo /etc/yum.repos.d/docker-ce.repo

# Install Docker and plugins
RUN dnf install -y \
    docker-ce \
    docker-ce-cli \
    containerd.io \
    docker-buildx-plugin \
    docker-compose-plugin \
    && dnf clean all

# Add a non-root user and enable them to use sudo without password
# RUN groupadd --gid $USER_GID $USERNAME \
#     && useradd -s /bin/bash --uid $USER_UID --gid $USER_GID -m $USERNAME \
#     && sed -i /etc/sudoers -re 's/^%sudo.*/%sudo ALL=(ALL:ALL) NOPASSWD: ALL/g' \
#     && sed -i /etc/sudoers -re 's/^root.*/root ALL=(ALL:ALL) NOPASSWD: ALL/g' \
#     && sed -i /etc/sudoers -re 's/^#includedir.*/## **Removed the include directive** ##"/g' \
#     && echo "$USERNAME ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers \
#     && echo "Customized the sudoers file for passwordless access to the $USERNAME user!" \
#     && echo "$USERNAME user:";  su - $USERNAME -c id

RUN groupadd --gid $USER_GID $USERNAME && \
    useradd --uid $USER_UID --gid $USER_GID -m $USERNAME && \
    usermod -aG wheel $USERNAME && \
    echo "$USERNAME ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers.d/$USERNAME && \
    chmod 0440 /etc/sudoers.d/$USERNAME

USER $USERNAME
WORKDIR /home/$USERNAME/

# Enable Docker daemon
COPY ./resources/docker-entrypoint.sh /usr/local/bin/
RUN sudo chmod +x /usr/local/bin/docker-entrypoint.sh

# Expose Docker port (optional)
EXPOSE 2375

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

################################################################################



ENTRYPOINT ["/usr/local/bin/docker-entrypoint.sh"]
################################################################################  
