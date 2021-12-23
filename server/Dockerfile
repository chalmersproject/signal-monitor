# Runner:
FROM node:16.13-bullseye

WORKDIR /server
COPY ./target/release/server/ ./
COPY ./app/ ./app/

# Configure environment
ENV HOST=0.0.0.0
ENV PORT=3000

# Configure entrypoint, healthcheck
EXPOSE $PORT
CMD ["/server/server"]
HEALTHCHECK CMD curl --fail --head http://localhost:$PORT/_health
