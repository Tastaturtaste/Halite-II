# Build docker image on linux (or wsl), otherwise symlinks will not work correctly.
# The resulting image can be used on windows.

FROM ubuntu:22.04 as base
# make ~/.profile available
SHELL ["/bin/bash", "--login", "-c"]
RUN apt-get update 
RUN apt-get upgrade -y 
RUN apt-get install -y curl ruby python2 nodejs ruby-dev build-essential libz-dev npm

COPY . halite-II/

WORKDIR /halite-II/website
RUN gem install bundler
RUN bundle update
RUN bundle install --path=vendor/bundle
RUN npm update
RUN npm install

WORKDIR /halite-II/libhaliteviz
RUN npm update
RUN npm install

WORKDIR /halite-II/tools/standalone_visualizer
RUN npm update 
RUN npm install

WORKDIR /halite-II/website
RUN bundle exec jekyll build

WORKDIR /halite-II/tools/standalone_visualizer
RUN npm run build

WORKDIR /halite-II/website
RUN npm run build

EXPOSE 4000
CMD ["/bin/bash", "--login", "-c", "bundle exec jekyll serve --host 0.0.0.0 --port 4000"]