# Build docker image on linux (or wsl), otherwise symlinks will not work correctly.
# The resulting image can be used on windows.

# FROM ruby:2.7.1 AS ruby1
# FROM ruby:2.7.1 AS ruby1
# COPY . halite-II/
# WORKDIR /halite-II/website
# RUN rm -rf ./vendor/bundle
# RUN gem install bundler
# RUN bundle install --path=vendor/bundle

# FROM node:9 as node1
# COPY --from=ruby1 /halite-II /halite-II
# WORKDIR /halite-II/website
# # Clear cache, node_modules and package-lock.json to circumvent max callstack size exceeded error
# #RUN npm cache clean --force
# RUN rm -rf node_modules package-lock.json
# RUN npm install
# WORKDIR /halite-II/libhaliteviz
# RUN npm install
# WORKDIR /halite-II/tools/standalone_visualizer
# RUN npm install

# FROM ruby1 AS ruby2
# COPY --from=node1 /halite-II /halite-II
# WORKDIR /halite-II/website
# RUN bundle exec jekyll build

# FROM node1 AS node2
# COPY --from=ruby2 /halite-II /halite-II
# WORKDIR /halite-II/tools/standalone_visualizer
# RUN npm run build
# WORKDIR /halite-II/website
# RUN npm run build

# FROM ruby2 AS ruby3
# COPY --from=node2 /halite-II /halite-II
# WORKDIR /halite-II/website
# EXPOSE 4000
# CMD ["bundle", "exec", "jekyll", "serve", "--host", "0.0.0.0", "--port", "4000"]

FROM ubuntu as base
# make ~/.profile available
SHELL ["/bin/bash", "--login", "-c"]
RUN apt-get update && apt-get upgrade -y && apt-get install -y curl gnupg2 python2
RUN gpg2 --keyserver hkp://keyserver.ubuntu.com --recv-keys 409B6B1796C275462A1703113804BB82D39DC0E3 7D2BAF1CF37B13E2069D6956105BD0E739499BDB
RUN \curl -sSL https://get.rvm.io | bash -s stable
RUN apt-get purge libssl-dev -y
RUN rvm pkg install openssl
RUN rvm install 2.7 --with-openssl-dir=/usr/local/rvm/usr
RUN PROFILE=~/.profile bash -c 'curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.3/install.sh | bash'
RUN nvm install 9

COPY . halite-II/
WORKDIR /halite-II/website
RUN rm -rf ./vendor/bundle
RUN gem install bundler
RUN bundle install --path=vendor/bundle

WORKDIR /halite-II/website
# Clear cache, node_modules and package-lock.json to circumvent max callstack size exceeded error
#RUN npm cache clean --force
RUN rm -rf node_modules package-lock.json
RUN npm install
WORKDIR /halite-II/libhaliteviz
RUN npm install
WORKDIR /halite-II/tools/standalone_visualizer
RUN npm install

WORKDIR /halite-II/website
RUN bundle exec jekyll build

WORKDIR /halite-II/tools/standalone_visualizer
RUN npm run build
WORKDIR /halite-II/website
RUN npm run build

WORKDIR /halite-II/website
EXPOSE 4000
CMD ["/bin/bash", "--login", "-c", "bundle exec jekyll serve --host 0.0.0.0 --port 4000"]