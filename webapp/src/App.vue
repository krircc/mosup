<template>
<div id = "home">
  
    <button @click="count++">{{ count + 1 }}</button>
    <input type="text" id="username" name="username" placeholder="用户名：11位以内，字符,数字组成" v-model="username"  required />
    <input type="email" id="email" name="email" placeholder="邮箱：格式：xxx@xxx.xx" v-model="email"  required />
    <input type="password" name="password" placeholder="密码：11位以内，字符,数字组成" v-model="password"  required/>
    <input type="password" name="password2" placeholder="确认密码" v-model="password2"  required/><br/>
    
    <input type="submit" id="submit" @click="signup" value="注册"/>
    <div>
      <p>{{doc}}</p>
    </div>

</div>
</template>

<script>
import { onMounted, reactive, toRefs } from "vue";
export default {
  setup() {
      const data = reactive({
          count: 1,
          username: "",
          email: "",
          password: "",
          password2: "",
          doc: []
      })
      onMounted(() => {
        console.log('mounted!')
                fetch( 'http://127.0.0.1:8080/', {
                  method: 'GET',
                  mode: 'cors' 
                }).then(response => response.json())
                .then(json => {
                  data.doc = json
                })
                .catch((e) => {
                  console.log(e)
                })
      })
      const signup = () => {
                let userdata = {
                    username: data.username,
                    email: data.email,
                    password: data.password
                };
                console.log(userdata)
                fetch("http://127.0.0.1:8080/signup", {
                  body: JSON.stringify(userdata), 
                  headers: {
                    'content-type': 'application/json'
                  },
                  method: 'POST',
                  mode: 'cors' 
                }).then(response => response.json())
                .then(json =>  {
                  console.log(json)
                  window.location.reload ( true )
                  // window.location.replace("/")
                })
                .catch((e) => {
                  console.log(e)
                })
      }

      return { 
        ...toRefs(data),
        signup
      }
  }
}
</script>

<style scoped>
button {
  color: red;
}
</style>