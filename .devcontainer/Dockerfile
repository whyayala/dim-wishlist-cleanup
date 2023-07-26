FROM ubuntu:18.04
WORKDIR /home/

COPY . .

RUN bash ./script.sh

ENV PATH="/root/.cargo/bin:$PATH"

# initialize ZSH
RUN /bin/zsh ~/.zshrc