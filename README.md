# muro [![Build Status](https://travis-ci.org/OUIRC/muro.svg?branch=master)](https://travis-ci.org/OUIRC/muro)

The muro library for modern community.
>this is muro-v2. the muro-v1 is [here](https://github.com/something-here/muro)

muro is single page webapp written in [actix-web](https://github.com/actix/actix-web) with vuejs.
- Async stable Actix-web framework 
- diesel, postgresql r2d2
- SPA CORS JWT
- Vuejs Parcel-bundler

## How To
    first create a name 'muro' postgresql database for this project.

### with docker

```
docker-compose up -d
```

## when development 
```bash
$ git clone https://github.com/OUISRC/muro.git
$ cd muro
$ cargo install diesel_cli --no-default-features --features postgres
$ diesel setup
$ cargo run

// another shell  nodejs(v8.9.4 on my machine)

$ cd muro/webapp
$ npm install
$ npm run dev
```
then open browser 'http://localhost:1234/'

## when production

```bash
$ git clone https://github.com/OUISRC/muro.git
$ cd muro
$ cargo install diesel_cli --no-default-features --features postgres
$ diesel setup
$ cd webapp
$ npm install
$ npm run build
$ cd ..
$ cargo run
```
then open broswer 'http://localhost:8000/'

### <a name="screenshots"> What it looks like </a>

<img alt="Home" height="300" width="500" src="https://raw.githubusercontent.com/OUISRC/muro/master/home.png">

<img alt="theme" height="300" width="500" src="https://raw.githubusercontent.com/OUISRC/muro/master/theme.png">

<img alt="publish" height="300" width="500" src="https://raw.githubusercontent.com/OUISRC/muro/master/publish.png">

<img alt="auth" height="300" width="500" src="https://raw.githubusercontent.com/OUISRC/muro/master/auth.png">


## Contribute
 
welcome to contribute !

