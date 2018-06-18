<template>
  <div id="home">
      <mnav id="mnav"></mnav>
      <div id="show">
            <span><img src="https://sfault-avatar.b0.upaiyun.com/327/537/3275374482-59ebf6fe6c1ce_huge256" /></span>
            <span id="community-name">{{community_name}}
              <button v-if="signin_user" @click="like">Like</button>
            </span>
            
      </div>
      <main>
        <div id="center">
            <div id="header">
                  <li  id="first"><router-link to="/">Best</router-link></li>
                  <li  ><router-link to="/new">New</router-link></li>
                  <li  ><router-link to="/hot">Hot</router-link></li>
                  <li  ><router-link to="/top">Top</router-link></li>
                  <li  ><router-link to="/care">Care</router-link></li>
                  <li  ><router-link to="/rise">Rise</router-link></li>
            </div>
            <div id="title">
                  <span id="topic">Theme</span>
                  <span id="right">
                      <span id="info">User</span>
                      <span id="info">View</span>
                      <span id="info">Reply</span>
                      <span>Activity</span>
                  </span>      
            </div>
            <div id="content">
                  <div id="items" v-for="(theme, index) in community_theme_list" :key="index">
                      <div id="item">
                        <span id="left"><a :href="'/a/' + community_name + '/theme/' + theme.id" title="theme.title"> {{ theme.title }} </a></span>
                        <span id="right">
                            <span id="info"><a :href="'/a/user/' + theme.user_id"> {{ theme.username }} </a></span>
                            <span id="info"> {{ theme.view_count }} </span>
                            <span id="info"> {{ theme.comment_count }} </span>
                            <span > {{ theme.rtime }} </span>
                        </span>
                      </div>
                  </div>
            </div>
        </div>
      </main>
  </div>
</template>

<script>
import URLprefix from '../../config'
import auth from '../../utils/auth'
import Mnav from '../../components/nav/Mnav'
export default {
  name: 'home',
  components: {
    "mnav": Mnav
  },
  data: function() {
    return {
      community_theme_list: '',
      signin_user: '',
      community_name:''
    }
  },
  mounted: function() {
    this.signin_user = JSON.parse(sessionStorage.getItem('signin_user'))
    this.community_name = this.$route.params.name
      fetch(URLprefix + 'api/community/'+ this.$route.params.name,{
          method: 'GET',
      }).then(response => response.json())
      .then(json => {
            console.log(json)
            this.community_theme_list = json.community_theme_list.reverse()
      }).catch((e) => {
        console.log(e)
      })
  },
  methods: {
    like() {
        let user_id = JSON.parse(sessionStorage.getItem('signin_user')).id
        let community_name = this.$route.params.name
        let data = { 
          user_id: user_id,
          community_name: community_name
        }
              fetch(URLprefix + 'dyn/community/like', {
                  body: JSON.stringify(data), 
                  headers: {
                    'content-type': 'application/json'
                  },
                  method: 'POST',
              }).then(response => response.json())
              .then(json => {
                  console.log(json)
              })
              .catch((e) => {
                console.log(e)
              })
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
#show {
    background-color: #f7cff7;
    margin-bottom: 0.5rem;
}
#show button {
    font-size: 1.5rem;
    vertical-align: middle;
    font-weight: bold;
    width: 4rem;
    background-color:aqua;
}
#community-name {
    margin-left: 3rem;
    font-size: 1.8rem;
    font-weight: bold;
    vertical-align: middle;
}
main {
    padding-bottom: 44px;
}
#center {
  box-shadow: 0 1px 1px #faf5f5;
  background-color: #ffffff;
}
#center #header {
  height: 50px;
  background-color: #faf5f5;
  box-shadow: 0 0 3px rgba(0,0,0,0.1), 0 -1px 1px rgba(0,0,0,0.1);
}
#center #header #first {
  margin-left: 1vw;
}
#center #header li {
  display: inline-block;
  line-height: 50px;
  margin-left: 4vw;
  font-size: 1.1rem;
  font-weight: bold;
}
#center #title {
  line-height: 44px;
  color: #333333;
  border-bottom: 1px solid rgb(212, 212, 212);
}
#center #right {
  float: right;
  padding-right: 1vw;
}
#center #title #right #info {
  padding-right: 4vw;
}
#center #items {
  line-height: 55px;
  border-bottom: 1px solid rgb(231, 238, 233);
}
#center #items a {
  color: #0541af;
}

#center #topic, #left {
  padding: 0 2vw 0 1vw;
}

#center #content #right #info {
  padding-right: 4.4vw;
}
@media only screen and (max-width: 600px) {
    img {
        vertical-align: middle;
        margin-left: 2.2vw;
        width: 4rem;
        height: 4rem;
    }
    main{
        margin: 0 auto;
        width: 95%;
    }
    #center #header li {
      margin-left: 2.2vw;
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
    main{
        margin: 0 auto;
        width: 80%;
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
    main {
        margin: 0 auto;
        width: 70%;
    }
}
</style>