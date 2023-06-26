FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y \
    git \
    openssh-server

RUN adduser --disabled-password git && \
    mkdir -p /home/git/.ssh && \
    chown -R git:git /home/git/.ssh && \
    chmod 700 /home/git/.ssh

# COPY ./scripts/output/server/authorized_keys /home/git/.ssh/authorized_keys

# RUN chown git:git /home/git/.ssh/authorized_keys && \
#     chmod 600 /home/git/.ssh/authorized_keys

RUN mkdir -p /srv/git && \
    git init --bare /srv/git/repoA.git && \
    git init --bare /srv/git/repoB.git && \
    chown -R git:git /srv/git

RUN mkdir -p /var/run/sshd

EXPOSE 22

CMD ["/usr/sbin/sshd", "-D"]
