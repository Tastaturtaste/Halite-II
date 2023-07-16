# Build docker image on linux (or wsl), otherwise symlinks will not work correctly.
# The resulting image can be used on windows.

FROM ubuntu:22.04 as base
# make ~/.profile available
SHELL ["/bin/bash", "--login", "-c"]
RUN apt-get update && apt-get upgrade -y && apt-get install -y curl ruby python2 nodejs ruby-dev build-essential libz-dev npm

COPY . halite-II/
WORKDIR /halite-II/website
RUN rm -rf ./vendor/bundle
RUN gem install bundler
RUN bundle config set --local path 'vendor/bundle'
RUN bundle update
RUN bundle install

WORKDIR /halite-II/website
# Clear node_modules and package-lock.json to circumvent max callstack size exceeded error
RUN rm -rf node_modules
RUN npm update && npm install

WORKDIR /halite-II/libhaliteviz
RUN rm -rf node_modules
RUN npm update && npm install

WORKDIR /halite-II/tools/standalone_visualizer
RUN rm -rf node_modules
RUN npm update && npm install

WORKDIR /halite-II/website
RUN bundle exec jekyll build

WORKDIR /halite-II/tools/standalone_visualizer
RUN npm run build

WORKDIR /halite-II/website
RUN npm run build

EXPOSE 4000
CMD ["/bin/bash", "--login", "-c", "bundle exec jekyll serve --host 0.0.0.0 --port 4000"]