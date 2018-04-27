<template>
    <div id="theme">
        <mnav id="mnav"></mnav>
        <div id="body">
            <div id="theme">
                <div id="title">
                    <h3> {{ theme.title }} </h3> 
                    <span id="info"> {{ theme.category }}</span> • 
                    <span id="info"><a :href="'/a/user/' + theme.user_id">{{ theme_user.username }}</a></span> •   
                    <span id="info">{{ theme.created_at }}</span>  
                </div>
                <div id="content" v-html="theme.content" ></div>
            </div>
            <hr>
            <div id="comment">
                <div id="count">Comment &nbsp; {{ theme.comment_count }} </div>
                <div v-for="(comment, index) in theme_comment" :key="index">
                    <div id="detail">
                        <div id="infos">
                            <span id="info" style="font-weight: bold; color: #D10303;">{{ index + 1 }} </span>
                            <span id="info"><a :href="'/a/user/' + comment.user_id">{{ comment.user_id }}</a></span> • <span id="info">{{ comment.created_at }}</span>
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
            theme_comment: '',
            signin_user: ''
        }
    },
    mounted: function() {
        if (sessionStorage.getItem('signin_user')){
            this.signin_user = JSON.parse(sessionStorage.getItem('signin_user'))
        }
        axios.get("http://localhost:8000/api/" + this.$route.params.id)
        .then((response) => {
            this.theme = response.data.theme
            this.theme_user = response.data.theme_user
            this.theme_comment = response.data.theme_comment
            console.log(response.data.theme)
            console.log(response.data.theme_user)
        })
        .catch((e) => {
            console.log(e)
        })
  },
  methods: {
    comment () {
      let comment = this.Textarea
      let theme_id = this.$route.params.id
      let user_id = JSON.parse(sessionStorage.getItem('signin_user')).id
      axios.post("http://localhost:8000/api/" + this.$route.params.id, {
          the_theme_id: theme_id,
          user_id: user_id,
          comment: comment
      })
      .then((response) => {
        console.log(response.data.message)
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
#body #theme > #title h3 { 
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
    #body  {
      margin: 0.5rem auto;
      width: 95%;
  }
}
@media only screen and (min-width: 600px) and (max-width: 1000px) {
    #body  {
      margin: 0 auto;
      width: 72%;
      padding-top: 77px;
  }
}
@media only screen and (min-width: 1000px) {
  #body  {
      margin: 0 auto;
      width: 66%;
      padding-top: 77px;
  }
}
</style>