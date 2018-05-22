<template>
    <div id="create">
        <mnav id="mnav"></mnav>
        <div id="content">
            <div id="top"><p>Crate Community</p></div><br/>
            <div id="topic-group">
                    <span  id="category">
                            <select name="community_category" v-model="CommunityCategory" id="category-control" >
                                <option value="muro">muro<span class="icon-arrow"></span></option>
                                <option v-bind:value="community_category" v-for="(community_category, index) in community_categorys" :key="index">
                                        {{community_category}}
                                </option>
                            </select>
                    </span>
                    <!-- <span id="title">
                            <input type="text" name="new_community_name" v-model="NewCommunityNmae" placeholder="Please input a new_community_name">
                    </span> -->
            </div>    
            <input type="text" name="community_name" v-model="CommunityNmae" placeholder="Please input community_name"><br><br>
            <button type="submit" id="submit" @click="create" ><span class="tip"> Create </span></button>
        </div>
    </div>
</template>

<script>
import axios from 'axios'
import Mnav from '../../components/nav/Mnav'
export default {
    name: 'create',
    components: {
        "mnav": Mnav
    },
    data () {
        return {
            CommunityCategory: '',
            CommunityNmae: '',
            NewCommunityNmae: '',
            community_categorys: ''
        }
    },
    mounted: function() {
        axios.get('http://localhost:8001/api/community_categorys')
        .then((response) => {
            this.community_categorys = response.data.community_categorys
            console.log(response.data.community_categorys)
        })
        .catch((e) => {
            console.log(e)
        })
    },
    methods: {
        create () {
            var community_category = this.CommunityCategory
            var community_name = this.CommunityNmae
            var user_id = JSON.parse(sessionStorage.getItem('signin_user')).id
            console.log(community_category)
            console.log(community_name)
            console.log(user_id)
            axios.post('http://localhost:8001/api/community_new', {
                community_category: community_category,
                create_user_id: user_id,
                community_name: community_name
            })
            .then(response => {
                window.location.reload ( true )
                this.$router.push("/a/community/"+ community_name)
            })
            .catch(e => {
                console.log(e)
            })
        }
    }
}
</script>

<style scoped>
#top {
    margin: 4px auto;
    width: 100%;
    line-height: 33px;
    background-color:rgb(231, 236, 235);
}

#topic-group {
   margin: 11px 0 11px 0;
}
#topic-group #category #category-control {
    background-color: #FFFFFF;
    border :1px solid #CAC1C1; /*Chrome和Firefox里面的边框是不一样的，所以复写了一下*/
    border: solid 1px #CAC1C1;
    text-align: center;
}
#topic-group #category #category-control, #topic-group input {
    height: 30px;
}

button {
    width:63px; 
    line-height:30px;
    background-color: aqua;
    border :1px solid #a39c9c;
}
@media only screen and (max-width: 600px) {
    #content  {
      margin: 0.5rem auto;
      width: 95%;
    }
    #topic-group #category #category-control, #topic-group input {
        width: 100%;
    }
}
@media only screen and (min-width: 600px) and (max-width: 1000px) {
    #content  {
        margin: 0 auto;
        width: 72%;
        padding-top: 77px;
    }
    #topic-group input {
        width: 72%;
        float: right;
    }
}
@media only screen and (min-width: 1000px) {
  #content  {
      margin: 0 auto;
      width: 66%;
      padding-top: 77px;
  }
  #topic-group input {
        width: 80%;
        float: right;
  }
}
</style>