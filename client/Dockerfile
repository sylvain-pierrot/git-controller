FROM debian:bullseye-slim

WORKDIR /root

RUN apt-get update && apt-get install -y \
    git \
    ssh

RUN mkdir -p /root/.ssh && \
    chmod 700 /root/.ssh

ARG CLIENT

# COPY scripts/output/client-${CLIENT} /root/.ssh/

RUN git config --global user.email "root@local.com" && \
    git config --global user.name "root"

RUN mkdir -p /root/repos && \
    git init /root/repos/repoA && \
    git init /root/repos/repoB

# RUN chmod 600 /root/.ssh/id* && \
#     chmod 644 /root/.ssh/id*.pub

# COPY scripts/client-entrypoint.sh /usr/local/bin/entrypoint.sh

# RUN chmod +x /usr/local/bin/entrypoint.sh

# ENTRYPOINT ["sh", "-c", "/usr/local/bin/entrypoint.sh"]
