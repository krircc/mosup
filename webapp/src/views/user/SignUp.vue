<template>
 <div id="signup">
   <div id="content">
        <div id="title">    
            <router-link to="/a/signin">登录</router-link>&emsp;|&emsp;
            <router-link to="/a/signup">注册</router-link> 
        </div> 
          <input type="text" id="username" name="username" placeholder="用户名：11位以内，字符,数字组成" v-model="Username"  required />
          <input type="email" id="email" name="email" placeholder="邮箱：格式：xxx@xxx.xx" v-model="Email"  required />
          <input type="password" name="password" placeholder="密码：11位以内，字符,数字组成" v-model="Password"  required/>
          <input type="password" name="confirm_password" placeholder="确认密码" v-model="ConfirmPassword"  required/><br/>
          
          <input type="submit" id="submit" @click="signup" value="注册"/>
    </div>
  </div>
</template>

<script>
/* eslint-disable */
import URLprefix from '../../config'
export default {
  name: 'signup',
  data () {
    return {
      Username: '',
      Email: '',
      Password: '',
      ConfirmPassword: ''
    }
  },
  methods: {
    signup () {
      let username = this.Username
      let email = this.Email
      let password = this.Password
      let confirm_password = this.ConfirmPassword
      let data = { 
          username: username,
          email: email,
          password: password,
          confirm_password: confirm_password
      }
      let getemail = document.getElementById("email")
      if (username.length == ''){
          alert("用户名不能为空.")
          return
      }else if (username.length > 11){
          alert("用户名：11位以内，字符,数字组成.")
          return
      }else if(!getemail.checkValidity()) {
          alert("请输入正确的Email地址.")
          return
      }else if(password == '') {
          alert("密码不能为空，请重新输入")
          return
      }else if(password != confirm_password) {
          alert("两次密码输入不同，请重新输入")
          return
      }else{
              fetch(URLprefix + 'api/signup', {
                  body: JSON.stringify(data), 
                  headers: {
                    'content-type': 'application/json'
                  },
                  method: 'POST',
                  mode: "cors"
              }).then(response => response.json())
              .then(json => {
                  if (json.status == 200) {
                        localStorage.setItem('token',json.token);
                        localStorage.setItem('signin_user',JSON.stringify(json.signin_user));
                        // window.location.reload ( true ); 
                        this.$router.push('/a/signin')
                    }else{
                        alert(json.message)
                    }
              })
              .catch((e) => {
                console.log(e)
              })
      }
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
#content {
  width: 18rem;
  margin: 0 auto;
  padding-top: 33px;
}
#title {
    text-align: center;
    padding: 0.5rem 0;
    font-size: 1.3rem;
    font-weight: bold;
    background-color:bisque;
}
input[type="text"],
input[type="email"],
input[type="password"] {
  margin: 6px auto auto;
  width: 18rem;
  height: 36px;
  border: none;
  border-bottom: 1px solid #AAA;
  font-size: 16px;
}
#submit  {
  margin: 6px 0;
  width: 18rem;
  height: 40px;
  background-color:rgb(248, 211, 166);
  border: none;
  border-radius: 2px;
   font-weight: bold;
  font-size: 1.1rem;
}
#content {
  margin: 0 auto;
  padding-top: 5rem;
}
@media only screen and (min-width: 1000px) {
    #content {
      margin: 0 auto;
      padding-top: 8rem;
    }
}
</style>