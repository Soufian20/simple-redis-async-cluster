FROM redis/redis-stack-server:7.0.6-RC4

COPY ./clusterinit.sh /entrypoint.sh
RUN chmod 755 /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]