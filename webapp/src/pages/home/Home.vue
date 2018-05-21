<template>
  <div id="home">
      <mnav id="mnav"></mnav>
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
                  <span id="title-topic">Theme</span>
                  <span id="right">
                      <span id="info">Community</span>
                      <span id="info">User</span>
                      <span id="info">View</span>
                      <span id="info">Reply</span>
                      <span>Activity</span>
                  </span>      
            </div>
            <div id="content">
                <div v-if="signin_user">
                    <div id="items" v-for="(theme, index) in theme_list" :key="index">
                          <div id="office" v-if="theme.community_name === 'Office'">
                              <span id="office-title"><a :href="'/a/'+ theme.community_name + '/theme/' + theme.id" title="theme.title"> {{ theme.title }} </a></span>
                              <span id="right">
                                  <span id="info"><a :href="'/a/community/' + theme.community_name">  {{ theme.community_name }} </a></span>
                                  <span id="info"><a :href="'/a/user/' + theme.user_id"> {{ theme.username }} </a></span>
                                  <span id="info"> {{ theme.view_count }} </span>
                                  <span id="info"> {{ theme.comment_count }} </span>
                                  <span > {{ theme.rtime }} </span>
                              </span>                        
                          </div>
                    </div>
                    <div id="items" v-for="(theme, index) in theme_list">
                          <div id="item" v-if="theme.community_name !== 'Office'">
                              <span id="item-title"><a :href="'/a/'+ theme.community_name + '/theme/' + theme.id" title="theme.title"> {{ theme.title }} </a></span>
                              <span id="right">
                                  <span id="info"><a :href="'/a/community/' + theme.community_name">  {{ theme.community_name }} </a></span>
                                  <span id="info"><a :href="'/a/user/' + theme.user_id"> {{ theme.username }} </a></span>
                                  <span id="info"> {{ theme.view_count }} </span>
                                  <span id="info"> {{ theme.comment_count }} </span>
                                  <span > {{ theme.rtime }} </span>
                              </span>
                          </div>
                    </div>
                </div>
                <div v-else>
                  <div id="items" v-for="(theme, index) in theme_list" :key="index">
                      <div id="office" v-if="theme.community_name === 'Office'">
                          <span id="office-title"><a :href="'/a/'+ theme.community_name + '/theme/' + theme.id" title="theme.title"> {{ theme.title }} </a></span>
                          <span id="right">
                              <span id="info"><a :href="'/a/community/' + theme.community_name">  {{ theme.community_name }} </a></span>
                              <span id="info"><a :href="'/a/user/' + theme.user_id"> {{ theme.username }} </a></span>
                              <span id="info"> {{ theme.view_count }} </span>
                              <span id="info"> {{ theme.comment_count }} </span>
                              <span > {{ theme.rtime }} </span>
                          </span>                        
                      </div>
                  </div>
                  <div id="items" v-for="(theme, index) in theme_list">
                      <div id="item" v-if="theme.community_name !== 'Office'">
                        <span id="item-title"><a :href="'/a/'+ theme.community_name + '/theme/' + theme.id" title="theme.title"> {{ theme.title }} </a></span>
                        <span id="right">
                            <span id="info"><a :href="'/a/community/' + theme.community_name">  {{ theme.community_name }} </a></span>
                            <span id="info"><a :href="'/a/user/' + theme.user_id"> {{ theme.username }} </a></span>
                            <span id="info"> {{ theme.view_count }} </span>
                            <span id="info"> {{ theme.comment_count }} </span>
                            <span > {{ theme.rtime }} </span>
                        </span>
                      </div>
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
      theme_list: '',
      signin_user: ''
    }
  },
  mounted: function() {
      if (sessionStorage.getItem('signin_user')){
              this.signin_user = JSON.parse(sessionStorage.getItem('signin_user'))
              let signin_user_id = JSON.parse(sessionStorage.getItem('signin_user')).id
              axios.post('http://localhost:8001/api/theme_list',{
                user_id: signin_user_id
              })
              .then((response) => {
                this.theme_list = response.data.theme_list.reverse()
              })
              .catch((e) => {
                console.log(e)
              })
      }else{
          let signin_user_id = 0;
          axios.post('http://localhost:8001/api/theme_list',{
            user_id: signin_user_id
          })
          .then((response) => {
            this.theme_list = response.data.theme_list.reverse()
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
#center #items #office, #center #items #item {
  line-height: 55px;
  border-bottom: 1px solid rgb(231, 238, 233);
}
#center #office {
  color: #b93bf3;
}
#center #items #item a {
  color: #0541af;
}

#center #title-topic, #office-title, #item-title {
  padding: 0 2vw 0 1vw;
}

#center #content #right #info {
  padding-right: 4.4vw;
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