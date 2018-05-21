<template>
    <div id="center">
      <mnav id="mnav"></mnav>
      <div id="show"><img src="https://sfault-avatar.b0.upaiyun.com/327/537/3275374482-59ebf6fe6c1ce_huge256" /></div>
      <div id="title">
          <ul>
              <li>Topic</li>
              <li>Comment</li>
              <li>Message</li>
              <li>Save</li>
          </ul>
      </div>
      <div id="body">
        <div id="container">
            <div id="left">
                <p><strong>{{ username }}</strong></p>
                <p><strong>{{ email }}</strong></p>
                <p>created_time : {{ created_time }}</p>

                <button id="submit" v-if="username == ''" @click="login">Login</button><br/>
                <button id="submit" v-if="username != ''" @click="update">Update Account</button><br/>
                <button id="submit" v-if="username != ''" @click="deleteme">Delete Account</button><br/>

                <div id="update" v-if="userupdate == true">
                  <p>Account Update</p> 
                    <input type="text" name="newname" placeholder="Newname" v-model="Newname"  required /><br/>
                    <input type="text" name="newmail" placeholder="Newmail" v-model="Newmail"  required /><br/>
                    <input type="password" name="newpassword" placeholder="Newpassword" v-model="Newpassword"  required/><br/>
                    <input type="password" name="confirm_newpassword" placeholder="Confirm Newpassword" v-model="ConfirmNewpassword"  required/><br/>
                    <button id="submit" @click="submitnow">UpdateNow</button>
              </div>
            </div>
            <div id="right">
                <p>My Topic</p>
                <p>My Comment</p>
                <p>My Message</p>
            </div>
        </div>
        
        

        
      </div>
    </div>
</template>

<script>
import axios from 'axios'
import auth from '../../utils/auth'
import Mnav from '../../components/nav/Mnav'
export default {
  name: 'center',
  components: {
    "mnav": Mnav
  },
  data: function() {
    return {
      email: '',
      username: '',
      created_time: '',
      Newname: '',
      Newmail: '',
      Newpassword: '',
      ConfirmNewpassword: '',
      userupdate: false
    }
  },
  mounted: function() {
      if (sessionStorage.getItem('token')){
        axios.get('http://localhost:8001/api/user_info', auth.getAuthHeader())
        .then((response) => {
            this.email =  response.data.current_user.email
            this.username =  response.data.current_user.username
            this.created_time =  response.data.current_user.created_at
            console.log(response.data.current_user)
            console.log(response.data.current_user.email)
        })
        .catch((e) => {
          console.log(e)
        })
      }
  },
  methods: {
    login() {
        window.location.reload ( true ); 
        this.$router.push('/a/access')
    },
    update() {
        this.userupdate = true
    },
    submitnow() {
        var user_id = JSON.parse(sessionStorage.getItem('signin_user')).id
        var newname = this.Newname
        var newmail = this.Newmail
        var newpassword = this.Newpassword
        var confirm_newpassword = this.ConfirmNewpassword
        axios.post('http://localhost:8001/api/user_update', {
            user_id: user_id,
            newname: newname,
            newmail: newmail,
            newpassword: newpassword,
            confirm_newpassword: confirm_newpassword
        })
        .then(response => {
          this.userupdate = false
          window.location.reload ( true )
        })
        .catch(e => {
          console.log(e)
        })
    },
    deleteme() {
        axios.get('http://localhost:8001/api/user_delete', auth.getAuthHeader())
        .then((response) => {
            sessionStorage.removeItem('token')
            sessionStorage.removeItem('signin_user')
            window.location.reload ( true ); 
            this.$router.push('/')
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
    background-color: aquamarine;
}
#title {
    line-height: 3.5rem;
    background-color: #faeaf5;
}
button {
    width: 7rem; 
    line-height:25px;
    background-color:#ffffff;
    border :1px solid #a39c9c;
}
@media only screen and (max-width: 600px) {
  img {
      margin: 0.3rem 1rem 0.1rem;
      width: 5rem;
      height: 5rem;
  }
  #title ul li {
      display: inline-block;
      padding-left: 1rem;
      font-weight: bold;
  }
  #body  {
      margin: 1rem;
      width: 95%;
  }
  #body #container #right {
      margin: 1rem auto;
  }
}
@media only screen and (min-width: 600px) and (max-width: 1000px) {
  #show {
      padding-top: 6rem;
  }
  img {
      margin-left: 14%;
      width: 8rem;
      height: 8rem;
  }
  #title ul {
      margin-left: 33%;
  }
  #title ul li {
      display: inline-block;
      padding-left: 2rem;
      font-weight: bold;
  }
  #body  {
      margin: 0 auto;
      width: 72%;
      padding-top: 2rem;
  }
  #body #container {
    display: flex;
    flex-flow: row;
  }
  #body #container #left {
      width: 33%;
      padding-right: 1rem;
  }
  #body #container #right {
      flex: 1;
  }
  #body #container #left #update {
    margin: 2rem auto;
  }
}
@media only screen and (min-width: 1000px) {
  #show {
      padding-top: 6rem;
  }
  img {
      margin-left: 17%;
      width: 8rem;
      height: 8rem;
  }
  #title ul {
      margin-left: 33vw;
  }
  #title ul li {
      display: inline-block;
      padding-left: 2rem;
      font-weight: bold;
  }
  #body  {
      margin: 0 auto;
      width: 66%;
      padding-top: 2rem;
  }

  #body #container {
    display: flex;
    flex-flow: row;
  }
  #body #container #left {
      width: 29%;
      padding-right: 1rem;
  }
   #body #container #left p, #body #container #left button {
      margin-bottom: 0.4rem;
  }
  #body #container #right {
      flex: 1;
  }
  #body #container #left #update {
    margin: 2rem auto;
  }
}
</style>