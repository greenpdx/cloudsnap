<template>
    <div id="theme">
        <mnav id="mnav"></mnav>
                    <div id="body">
                        <div id="title">
                            <h3> {{ theme.title }} </h3> 
                            <span id="info"> {{ theme.category }}</span> • 
                            <span id="info"><a :href="'/a/user/' + theme.user_id">{{ theme_user.username }}</a></span> •   
                            <span id="info">{{ theme.created_at }}</span>  
                        </div>
                        <hr id="hr">
                        <div id="content">{{ theme.content }}</div>
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
            theme_user: ''
        }
    },
    mounted: function() {
    // let theme = this.$route.params.id
    // console.log(theme)
    axios.get("http://localhost:8000/api/" + this.$route.params.id)
      .then((response) => {
        this.theme = response.data.theme
        this.theme_user = response.data.theme_user
        console.log(response.data.theme)
        console.log(response.data.theme_user)
      })
      .catch((e) => {
        console.log(e)
      })
  }
}
</script>

<style scoped>
#body {
    background-color: #ffffff;
}
#content {
    margin: 2rem auto;
}
a {
    color: #0541af;
}
#body #title {
    margin-top: 2px;
    padding: 10px;
}
#body #title h3 { 
    padding-bottom: 0.3rem;
}
#body #title #info {
    display: inline-block;
    font-size: 14px;
}
#hr {
    height: 6px;
    background-color: #faf5f5;
    border: 0;
}
#body #content {
    margin: 10px;
    padding: 5px;
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