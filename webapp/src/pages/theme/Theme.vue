<template>
    <div id="theme">
        <mnav id="mnav"></mnav>
        <div id="show">
            <span><img src="https://sfault-avatar.b0.upaiyun.com/327/537/3275374482-59ebf6fe6c1ce_huge256" /></span>
            <span id="community-name">{{theme_community_name}}</span>
        </div>
        <div id="body">
            <div id="theme">
                <div id="title">
                    <h2> {{ theme.title }} </h2> 
                    <span id="info"><a :href="'/a/community/' + theme_community_name">{{ theme_community_name }}</a></span> • 
                    <span id="info"><a :href="'/a/user/' + theme_user.user_id">{{ theme_user.username }}</a></span> •   
                    <span id="info">{{ theme_rtime }}</span>  
                </div>
                <div id="content" v-html="theme.content" ></div>
            </div>
            <hr>
            <div id="comment">
                <div id="count" style="font-weight: bold; color: #b93bf3;">Comment &nbsp; {{ theme.comment_count }} </div>
                <div v-for="(comment, index) in theme_comments" :key="index">
                    <div id="detail">
                        <div id="infos">
                            <span id="info" >{{ index + 1 }} </span>
                            <span id="info"><a :href="'/a/user/' + comment.user_id">{{ comment.username }}</a></span> • <span id="info">{{ comment.rtime }}</span>
                        </div>
                        <div id="content" v-html="comment.content" > </div>
                    </div>
                </div>
            </div>
            <hr>
            <div id="reply" v-if="signin_user">
                <div id="write"> Write comment in markdwon! </div>
                <textarea name="comment" v-model="Textarea" placeholder="if you want @somebody for send a message in your comment, the rule is: 
                1: the @ symbol can't be first position at line.(like: @somebodyxxxxx)
                2: one position before the @ symbol can't be space(like: xxxxx @somebodyxxxxx)."></textarea><br>
                <button id="submit" @click="comment">Comment</button>
            </div>  
            <div v-else style="margin: 10px;">Please login first and make a Comment.
                <a href="/a/access" style="background-color:aqua;">Login</a>
            </div>    
        </div>
    </div>
</template>

<script>
import axios from 'axios'
import URLprefix from '../../config'
import Mnav from '../../components/nav/Mnav'
export default {
    name: 'theme',
    components: {
        "mnav": Mnav
    },
    data: function() {
        return {
            theme: '',
            theme_user: '',
            theme_community_name: '',
            theme_rtime: '',
            theme_comments: '',
            signin_user: ''
        }
    },
    mounted: function() {
        if (sessionStorage.getItem('signin_user')){
            this.signin_user = JSON.parse(sessionStorage.getItem('signin_user'))
        }
        fetch(URLprefix + 'api/'+ this.$route.params.id,{
            method: 'GET',
        }).then(response => response.json())
        .then(json => {
            this.theme = json.theme
            this.theme_user = json.theme_user
            this.theme_rtime = json.theme_rtime
            this.theme_community_name = json.theme_community_name
            this.theme_comments = json.theme_comments
        }).catch((e) => {
            console.log(e)
        })
  },
  methods: {
    comment () {
      let comment = this.Textarea
      let theme_id = this.$route.params.id
      let user_id = JSON.parse(sessionStorage.getItem('signin_user')).id
      let data = {
          the_theme_id: theme_id,
          user_id: user_id,
          comment: comment
      }
              fetch(URLprefix + 'api/' + this.$route.params.id, {
                  body: JSON.stringify(data), 
                  headers: {
                    'content-type': 'application/json'
                  },
                  method: 'POST',
              }).then(response => response.json())
              .then(json => {
                  console.log(json)
                  window.location.reload ( true )
              })
              .catch((e) => {
                console.log(e)
              })
    }
  }
}
</script>

<style scoped>
#show {
    background-color: #f7cff7;
    margin-bottom: 0.5rem;
}
#community-name {
    margin-left: 3rem;
    font-size: 1.8rem;
    font-weight: bold;
    vertical-align: middle;
}
#body {
    background-color: #ffffff;
}
a {
    color: #0541af;
}
#body #theme > #title {
    margin-top: 2px;
    padding: 10px;
    border-bottom: 1px solid rgb(223, 223, 223);
}
#body #theme > #title h2 { 
    padding-bottom: 0.3rem;
}
#body #theme > #title #info {
    display: inline-block;
    font-size: 14px;
}
#body #theme > #content {
    margin: 10px;
}
hr {
    height: 11px;
    background-color: #faf5f5;
    border: 0;
}
#body #comment > #count {
    padding: 10px;
    border-bottom: 1px solid rgb(223, 223, 223);
}
#body #comment #detail {
    border-bottom: 1px solid rgb(223, 223, 223);
}
#body #comment #detail #infos{
    margin: 10px;
    margin-bottom: 10px;
}
#body #comment #detail #info{
    display: inline-block;
    font-size: 14px;
}
#body #comment #detail #content {
    margin: 10px;
}
#body #reply {
    margin: 10px;
}
#body #reply #write {
    margin-bottom: 10px;
}
#reply textarea {
    width:100%; 
    height: 200px;
}
#body #reply button {
    width:66px; 
    line-height:25px;
    background-color:#ffffff;
    border :1px solid #a39c9c;
}
@media only screen and (max-width: 600px) {
    img {
        vertical-align: middle;
        margin-left: 2.2vw;
        width: 4rem;
        height: 4rem;
    }
    #body  {
      margin: 0.5rem auto;
      width: 95%;
  }
}
@media only screen and (min-width: 600px) and (max-width: 1000px) {
    #show {
        padding-top: 4rem;
    }
    img {
        vertical-align: middle;
        margin-left: 10%;
        width: 6rem;
        height: 6rem;
    }
    #body  {
      margin: 0 auto;
      width: 72%;
  }
}
@media only screen and (min-width: 1000px) {
    #show {
        padding-top: 4rem;
    }
    img {
        vertical-align: middle;
        margin-left: 15%;
        width: 8rem;
        height: 8rem;
    }
    #body  {
        margin: 0 auto;
        width: 66%;
    }
}
</style>