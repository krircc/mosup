<template>
  <div id="signin">
      <div id="content">
          <div id="title">    
            <router-link to="/a/signin">登录</router-link>&emsp;|&emsp;
            <router-link to="/a/signup">注册</router-link> 
          </div>
            <input type="text" name="username" placeholder="用户名" v-model="Username" />
            <input type="password" name="password" placeholder="密码" v-model="Password" /><br/>
          <div>
              <div id="v_container" style="height: 44px;"></div>
              <input type="text" id="code_input" value="" placeholder="请输入上方验证码" style="width: 70%;"/>
              <span><button id="verify" >点击验证</button></span>
          </div>
          <button id="submit" @click="signin">登录</button><br/>
          <p id="tip">若未注册请先-<a href="/a/signup">注册</a>&emsp;&emsp;若忘密码请-<a href="/a/userset/password">找回</a></p>
      </div>
  </div>
</template>

<script>
/* eslint-disable */
import URLprefix from '../../config'
import  '../../../static/js/gVerify.js'
import Mnav from '../../components/nav/Mnav.vue'
export default {
  name: 'signin',
  components: {
    "mnav": Mnav
  },
  data () {
    return {
      Username: '',
      Password: ''
    }
  },
  mounted: function() {
    var verifyCode = new GVerify("v_container");
    document.getElementById("verify").onclick = function () {
      var res = verifyCode.validate(document.getElementById("code_input").value);
      if (res) {
        let verify = document.getElementById("verify")
        verify.innerHTML = "验证成功"
      } else {
        let verify = document.getElementById("verify")
        verify.innerHTML = "重新验证"
      }
    }
  },
  methods: {
    signin () {
      let uname = this.Username
      let password = this.Password
      let data = { 
          username: uname,
          password: password,
          code: "00",
      }
      if (document.getElementById("verify").innerHTML == "验证成功") {
            fetch(URLprefix + 'api/signin', {
                  body: JSON.stringify(data), 
                  headers: {
                    'content-type': 'application/json'
                  },
                  method: 'POST',
                  mode: 'cors'
            }).then(response => response.json())
            .then(json => {
                    if (json.status == 200) {
                        localStorage.setItem('token',json.token);
                        localStorage.setItem('signin_user',JSON.stringify(json.signin_user));
                        this.$router.push('/');
                        window.location.reload ( true );
                    }else{
                        alert(json.message)
                    }
            })
            .catch((e) => {
                console.log(e)
            })
      }else{
          alert("请先成功通过验证码再登录.")
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
input[type="password"] {
  margin: 6px auto auto;
  width: 18rem;
  height: 36px;
  border: none;
  border-bottom: 1px solid #AAA;
  font-size: 16px;
}
#verify {
  width: 30%; 
  padding: 6px 0;
  font-size: 1rem;
  background-color: bisque;
  border: none;
}
#submit  {
  margin: 6px 0;
  width: 18rem;
  height: 40px;
  background-color:rgb(250, 212, 165);
  border: none;
  border-radius: 2px;
  font-weight: bold;
  font-size: 1.1rem;
}
#content {
  margin: 0 auto;
  padding-top: 5rem;
}
#content #tip {
    background-color: #feffef;
    
}
#content #tip a {
    color: #0136f5;
    font-weight: bold;
}
@media only screen and (min-width: 1000px) {
    #content {
      margin: 0 auto;
      padding-top: 8rem;
    }
}
</style>