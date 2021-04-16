# Setup Stage - set up the ZSH environment for optimal developer experience
FROM node:14-alpine AS setup
# Let scripts know we're running in Docker (useful for containerised development)
ENV RUNNING_IN_DOCKER true
# Use the unprivileged `node` user (pre-created by the Node image) for safety (and because it has permission to install modules)
RUN mkdir -p /app \
    && chown -R node:node /app
# Set up ZSH and our preferred terminal environment for containers
RUN apk --no-cache add zsh curl git
RUN mkdir -p /home/node/.antigen
RUN curl -L git.io/antigen > /home/node/.antigen/antigen.zsh
# Use my starter Docker ZSH config file for this, or your own ZSH configuration file (https://gist.github.com/arctic-hen7/bbfcc3021f7592d2013ee70470fee60b)
COPY .dockershell.sh /home/node/.zshrc
RUN chown -R node:node /home/node/.antigen /home/node/.zshrc
# Set up ZSH as the unprivileged user (we just need to start it, it'll initialise our setup itself)
USER node
RUN /bin/zsh /home/node/.zshrc
# Switch back to root for whatever else we're doing
USER root

# Elm Setup Stage - install and set up Elm for development (used for frontend)
FROM setup AS elm-setup
# Download the Elm compiler, unzipping it directly and moving it into our binaries directory (even as root, this will work for `node`)
RUN curl -L -o - "https://github.com/elm/compiler/releases/download/0.19.1/binary-for-linux-64-bit.gz" \
	| gunzip > /usr/local/bin/elm
RUN chmod +x /usr/local/bin/elm

# Rust Setup Stage - install and set up Rust for development (used for backend)
FROM elm-setup AS rust-setup
# Install the necessary system dependencies
RUN apk add --no-cache build-base clang llvm gcc
# Download and run the Rust installer, using the default options (needs to be done as the unprivileged user)
USER node
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN source /home/node/.cargo/env
# Switch back to root for the remaining stages
USER root

# Base Stage - install system-level dependencies, disable telemetry, and copy files
FROM rust-setup AS base
WORKDIR /app
# Disable telemetry of various tools for privacy
RUN yarn config set --home enableTelemetry 0
# Copy our source code into the container
COPY . .

# Playground stage - simple ZSH entrypoint for us to shell into the container as the non-root user (enters in `/app`)
FROM base AS playground
USER node
ENTRYPOINT [ "/bin/zsh" ]
