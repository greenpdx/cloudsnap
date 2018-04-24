<template>
  <div id="home">
      <mnav id="mnav"></mnav>
      <main>
        <div id="center">
            <div id="header">
                  <li  id="first"><router-link to="/">All</router-link></li>
                  <li  ><router-link to="/latest">Latest</router-link></li>
                  <li  ><router-link to="/top">Top</router-link></li>
                  <li  ><router-link to="/share">Share</router-link></li>
                  <li  ><router-link to="/blog">Blog</router-link></li>
                  <li  ><router-link to="/help">Help</router-link></li>
                  <li  ><router-link to="/job">Job</router-link></li>
            </div>
            <div id="title">
                  <span id="title-topic">Theme</span>
                  <span id="right">
                      <span id="info">Category</span>
                      <span id="info">User</span>
                      <span id="info">View</span>
                      <span id="info">Reply</span>
                      <span>Activity</span>
                  </span>      
            </div>
            <div id="content">
                  <div id="items" v-for="(theme, index) in theme_list" :key="index">
                      <div id="office" v-if="theme.category === 'Office'">
                          <span id="office-title"><a :href="'/a/theme/' + theme.id" title="theme.title"> {{ theme.title }} </a></span>
                          <span id="right">
                              <span id="info"> {{ theme.category }} </span>
                              <span id="info"><a :href="'/a/user/' + theme.user_id"> {{ theme.username }} </a></span>
                              <span id="info"> {{ theme.id }} </span>
                              <span id="info"> {{ theme.id }} </span>
                              <span > {{ theme.created_at }} </span>
                          </span>                        
                      </div>
                  </div>
                  <div id="items" v-for="(theme, index) in theme_list">
                      <div id="item" v-if="theme.category !== 'Office'">
                        <span id="item-title"><a :href="'/a/theme/' + theme.id" title="theme.title"> {{ theme.title }} </a></span>
                        <span id="right">
                            <span id="info"> {{ theme.category }} </span>
                            <span id="info"><a :href="'/a/user/' + theme.user_id"> {{ theme.username }} </a></span>
                            <span id="info"> {{ theme.id }} </span>
                            <span id="info"> {{ theme.id }} </span>
                            <span > {{ theme.created_at }} </span>
                        </span>
                      </div>
                  </div>
            </div>
        </div>
      </main>
  </div>
</template>

<script>
import axios from 'axios'
import auth from '../../utils/auth'
import Mnav from '../../components/nav/Mnav'
export default {
  name: 'home',
  components: {
    "mnav": Mnav
  },
  data: function() {
    return {
      theme_list: ''
    }
  },
  mounted: function() {
    axios.get('http://localhost:8000/api/theme_list', auth.getAuthHeader())
      .then((response) => {
        this.theme_list = response.data.theme_list.reverse()
        console.log(response.data.theme_list)
        console.log(sessionStorage.getItem('token'))
        console.log(JSON.parse(sessionStorage.getItem('signin_user')).username)
      })
      .catch((e) => {
        console.log(e)
      })
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
main {
    padding-bottom: 44px;
}
#center {
  box-shadow: 0 1px 1px #cccccc;
}
#center #header {
  height: 50px;
  background-color: rgb(215, 248, 237); 
  /* box-shadow: 0 -3px 3px rgba(0,0,0,0.12), 0 -1px 1px rgba(0,0,0,0.24); */
}
#center #header #first {
  margin-left: 1vw;
}
#center #header li {
  display: inline-block;
  line-height: 50px;
  margin-left: 5vw;
  font-weight: bold;
}
#center #title {
  line-height: 44px;
  border-bottom: 1px solid rgb(231, 238, 233);
}
#center #title #right #info {
  padding-right: 4vw;
}
#center #items #office, #center #items #item {
  line-height: 55px;
  border-bottom: 1px solid rgb(231, 238, 233);
}
#center #office {
  color: #eb15ce;
}
#center #items #item a {
  color: #0541af;
}

#center #title-topic, #office-title, #item-title {
  padding: 0 2vw 0 1vw;
}
#center #right {
  float: right;
  padding-right: 1vw;
}
#center #right #info {
  padding-right: 3vw;
}
@media only screen and (max-width: 600px) {
    main{
        margin: 0 auto;
        width: 95%;
    }
    #center {
        margin-top: 28px;
    }
    #center #header li {
      margin-left: 2.2vw;
    }
}
@media only screen and (min-width: 600px) and (max-width: 1000px) {
    main{
        margin: 0 auto;
        width: 80%;
        padding-top: 77px;
    }
}
@media only screen and (min-width: 1000px) {
    main {
        margin: 0 auto;
        width: 70%;
        padding-top: 77px;
    }
}
</style>