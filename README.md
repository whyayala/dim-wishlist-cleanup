![Build](https://github.com/whyayala/dim-wishlist-cleanup/actions/workflows/main.yml/badge.svg)

# Tools

- [AWS CLI](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html)

- Rust
  - Pest for parsing
# Setup

`curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip"`
`unzip awscliv2.zip && sudo ./aws/install`

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

`script/get_voltron.sh`

# Run

`cargo run`

# Grammar Testing

https://pest.rs/#editor


# Notes

## Combatant Damage Scaling

### Adds

The strongest primary ammo weapons against minors grouped by starting range damage falloff, then by type, and ordered based on the subtype's "simple dps (all headshots, no reload)": 

Close Range (Sidearms, SMGs, Auto Rifles)

1. Rapid Fire Sidearm [568]
1. Adaptive Burst Sidearm [565]
1. Lightweight Sidearm [563]
1. Adaptive Sidearm [545]

1. Lightweight SMG [513]
1. Adaptive Frame SMG [481]
1. Aggressive Frame SMG [479]

1. Rapid Fire Auto Rifle [432]
1. Adaptive Auto Rifle [418]
1. Precision Auto Rifle [418]

Mid Range (Hand Cannons, Pulse Rifles)

1. Heavy Burst Hand Cannon [591]
1. Precision Hand Cannon [551]
1. Adaptive Hand Cannon [499]

1. Rapid Fire Pulse Rifle [429]
1. Adaptive Pulse Rifle [410]

