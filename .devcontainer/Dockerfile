FROM rust:1.81.0

ARG USER_ID
ARG GROUP_ID
ARG USER_NAME

RUN groupadd -g ${GROUP_ID} ${USER_NAME} && \
    useradd -m -u ${USER_ID} -g ${USER_NAME} ${USER_NAME}

RUN apt update && apt install git bash-completion openssh-client vim tree -y && apt clean

# 環境整備
RUN echo "source /usr/share/bash-completion/completions/git" >> /home/${USER_NAME}/.bashrc \
 && echo "source /etc/bash_completion.d/git-prompt" >> /home/${USER_NAME}/.bashrc
RUN echo "PROMPT_COMMAND='PS1_CMD1=\$(__git_ps1 \" (%s)\")'; PS1='\[\e[38;5;40m\]\u@\h\[\e[0m\]:\[\e[38;5;39m\]\w\[\e[38;5;214m\]\${PS1_CMD1}\[\e[0m\]\\$ '" >> /home/${USER_NAME}/.bashrc
RUN chsh -s /bin/bash ${USER_NAME}

# atcoder対応
RUN rustup toolchain install 1.70.0 \
 && rustup default 1.70.0 \
 && cargo install --locked cargo-compete cargo-member
RUN rustup component add rustfmt clippy rust-src

RUN chown -R ${USER_NAME}:${USER_NAME} /usr/local/cargo
USER ${USER_NAME}

WORKDIR /opt