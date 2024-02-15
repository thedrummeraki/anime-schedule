FROM ruby:3.2.0

WORKDIR /app

ENV BUNDLE_PATH=/gems
VOLUME [ "/gems" ]

COPY Gemfile /app/Gemfile
COPY Gemfile.lock /app/Gemfile.lock

RUN bundle install --jobs -5 --retry 4

COPY . /app

ENTRYPOINT [ "bundle", "exec" ]

