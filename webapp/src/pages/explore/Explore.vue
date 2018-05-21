<template>
  <div  id="explore">
      <mnav id="mnav"></mnav>
      <div id="content">
          <div id="community">
                <div id="title" style="font-weight: bold; color: #b93bf3;">Communitys </div>
                <div v-for="(community_category, index) in community_category_list" :key="index">
                  <div id="community_category" >{{ community_category }}</div>
                  <div v-for="(community, index) in communitys" :key="index">
                    <div v-if="community.community_category == community_category">
                      <div id="detail">
                          <div id="community_name" > <a :href="'/a/community/' + community.community_name">{{ community.community_name }} </a></div>
                          <div id="infos">
                              <span id="info"><a :href="'/a/user/' + community.create_user_id"> {{ community.create_user_id }} </a></span> • 
                              <span id="info"><a :href="'/a/community/' + community.community_category"> {{ community.community_category }} </a></span> •  
                              <span id="info"> {{ community.created_at }}</span>
                          </div>
                      </div>
                    </div>
                  </div>
                </div>
          </div>
      </div>
  </div>
</template>

<script>
import axios from 'axios'
import Mnav from '../../components/nav/Mnav'
export default {
  name: 'explore',
  components: {
    "mnav": Mnav
  },
  data: function() {
    return {
      communitys:'',
      community_category_list: ''
    }
  },
  mounted: function() {
    axios.get("http://localhost:8001/api/communitys")
      .then((response) => {
        this.communitys = response.data.communitys
        console.log(response.data.communitys)
        let community_category_result = {}
        for (let index = 0; index < response.data.communitys.length; index++) {
          community_category_result[response.data.communitys[index].community_category] = response.data.communitys[index].community_category
        }
        let community_category_list = new Array(); 
        for(var key in community_category_result){ 
            community_category_list.push(key); 
        } 
        this.community_category_list = community_category_list
        console.log(community_category_list)
        
      })
      .catch((e) => {
        console.log(e)
      })
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
#content #community  #title {
    border-bottom: 1px solid rgb(223, 223, 223);
}
#content #community #community_category {
   font-weight: bold;
    margin: 10px 0;
    background-color: aquamarine;
}
#content #community #detail {
    margin: 10px 0 10px 33px;
    border-bottom: 1px solid rgb(223, 223, 223);
}
#content #community #detail #community_name {
    font-weight: bold;
    color: #261cb6;
}
#content #community #detail #infos{
    margin: 10px 0;
    margin-bottom: 10px;
}
#content #community #detail #info{
    display: inline-block;
    font-size: 14px;
}
@media only screen and (max-width: 600px) {
    #content  {
      margin: 0.5rem auto;
      width: 95%;
  }
}
@media only screen and (min-width: 600px) and (max-width: 1000px) {
    #content  {
      margin: 0 auto;
      width: 72%;
      padding-top: 77px 0 22px 0;
  }
}
@media only screen and (min-width: 1000px) {
  #content  {
      margin: 0 auto;
      width: 66%;
      padding: 77px 0 22px 0;
  }
}
</style>