# muro [![Build Status](https://travis-ci.org/OUIRC/muro.svg?branch=master)](https://travis-ci.org/OUIRC/muro)

The interest and community for internet .
>this is muro-v2. the muro-v1 is [here](https://github.com/something-here/muro)

muro is single page webapp written in [actix-web](https://github.com/actix/actix-web) with vuejs.
- Async stable Actix-web framework 
- diesel, postgresql r2d2
- SPA CORS JWT
- Vuejs

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

// another shell  nodejs(v10.1.0 on my machine)

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

<img alt="Home" width="900" src="https://raw.githubusercontent.com/OUISRC/muro/master/home.png">

<img alt="explore" width="900" src="https://raw.githubusercontent.com/OUISRC/muro/master/explore.png">

<img alt="community" width="900" src="https://raw.githubusercontent.com/OUISRC/muro/master/community.png">

<img alt="theme" width="900" src="https://raw.githubusercontent.com/OUISRC/muro/master/theme.png">

<img alt="post" width="900" src="https://raw.githubusercontent.com/OUISRC/muro/master/post.png">

<img alt="user" width="900" src="https://raw.githubusercontent.com/OUISRC/muro/master/user.png">

<img alt="auth" width="900" src="https://raw.githubusercontent.com/OUISRC/muro/master/auth.png">


## Contribute
 
welcome to contribute !

