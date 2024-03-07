## update and install some things we should probably have
apt update
apt install -y curl \
  git \
  gnupg2 \
  jq \
  sudo \
  zsh \
  vim \
  build-essential \
  openssl

## Install rustup and common components
curl https://sh.rustup.rs -sSf | sh -s -- -y 

# Path to rustup and cargo
RUSTUP="/root/.cargo/bin/rustup"
CARGO="/root/.cargo/bin/cargo"

$RUSTUP install nightly
$RUSTUP component add rustfmt
$RUSTUP component add rustfmt --toolchain nightly
$RUSTUP component add clippy 
$RUSTUP component add clippy --toolchain nightly

## setup and install oh-my-zsh
sh -c "$(curl -fsSL https://raw.githubusercontent.com/robbyrussell/oh-my-zsh/master/tools/install.sh)"
cp -R /root/.oh-my-zsh /home/$USERNAME
cp /root/.zshrc /home/$USERNAME
sed -i -e "s/\/root\/.oh-my-zsh/\/home\/$USERNAME\/.oh-my-zsh/g" /home/$USERNAME/.zshrc
chown -R $USER_UID:$USER_GID /home/$USERNAME/.oh-my-zsh /home/$USERNAME/.zshrc
