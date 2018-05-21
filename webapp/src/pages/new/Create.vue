<template>
    <div id="create">
        <mnav id="mnav"></mnav>
        <div id="content">
            <div id="title"><p>Crate Community</p></div><br/>
            <input type="text" name="community_name" placeholder="community_name" v-model="CommunityNmae" /><br/><br/>
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
            CommunityNmae: ''
        }
    },
    methods: {
        create () {
            var community_name = this.CommunityNmae
            var user_id = JSON.parse(sessionStorage.getItem('signin_user')).id
            axios.post('http://localhost:8001/api/community_new', {
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
#title {
    margin: 4px auto;
    width: 100%;
    line-height: 33px;
    background-color:rgb(231, 236, 235);
}

button {
    width:63px; 
    line-height:30px;
    background-color:rgb(255, 255, 255);
    border :1px solid #a39c9c;
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
        padding-top: 77px;
    }
}
@media only screen and (min-width: 1000px) {
  #content  {
      margin: 0 auto;
      width: 66%;
      padding-top: 77px;
  }
}
</style>