<template>
    <div id="mnav">
      <div id="lnav">
        <div id="left">
          <span id="logo"><a id="front" href="/"><img src="../../../static/imgs/kriry.png"/></a></span>
          <span id="dlnav">
            <!-- <a id="name" href="/">Ouisrc</a>
            <a id="tail" href="/a/wiki" title="wiki">推荐</a>
            <a id="tail" href="/a/interest" title="interest">兴趣</a> -->
            <li>
                <div class="dropdown">
                    <div id="dropbox"><button class="dropbtn" @click="myFunction">Home<span class="arrow"></span></button></div>
                    <div class="dropdown-content" id="myDropdown">
                        <a href="/">Home</a>
                        <a :href="'/a/community/' + community_name.name" v-bind:value="community_name" v-for="(community_name, index) in community_names" :key="index">
                              {{community_name.name_cn}}
                        </a>
                    </div>
                </div>
            </li>
            <li>
                <a id="tail" href="/a/explore" title="explore">发现</a>
            </li>
            <!-- <a id="tail" href="">search</a> -->
          </span>
        </div>
        <label id="menu"><img src="../../../static/imgs/menu.jpg" ></label>
      </div>
      <div id="rnav">
          <div id="mlnav"> 
            <!-- <a id="tail" href="/">首页</a> -->
            <!-- <a id="tail" href="">搜索</a> -->
          </div>
          <li v-if="signin_user.username"> 
            <a v-if="messages_count != 0" :href="'/a/user/' + signin_user.id + '/message'" id="mnumber">{{messages_count}}</a>
          </li>
          <li v-if="signin_user.username == 'admin'"> 
            <a href="/a/create" title="create">新建</a>
            <a href="/a/tag" title="create">标签</a>
          </li>
          <li v-if="signin_user.username"> 
            <a href="/a/post" title="post">发布</a>
            <a :href="'/a/user/' + signin_user.id" title="signin_userusername">{{signin_user.username}}</a>
            <a href="/a/signin" title="Logout" @click="logout">退出</a>
          </li>
          <li v-else > 
            <a href="/a/signin" title="signin">登录</a>
            <a href="/a/signup" title="signin">注册</a>
          </li>
      </div> 
    </div>
</template>

<script>
/* eslint-disable */
import URLprefix from '../../config'
export default {
  name: 'Mnav',
  data: function () {
    return { 
      community_names: '',
      CommunityName: '',
      signin_user: '',
      messages_count:''
    }
  },
  mounted: function() {

        // 点击下拉菜单意外区域隐藏
      window.onclick = function(e) {
          if (!e.target.matches('.dropbtn')) {
              var myDropdown = document.getElementById("myDropdown");
              if (myDropdown.classList.contains('show')) {
                  myDropdown.classList.remove('show');
              }
          }
      };

      let menu = document.getElementById('menu');
      menu.addEventListener('click', function() {
          let nav = document.getElementById('rnav');
              if (nav.style.height == 'auto') {
                  nav.style.height = '0';
              }else{
                  nav.style.height = 'auto';
              }
      }, false);

      if (JSON.parse(localStorage.getItem('signin_user'))) {
          this.signin_user = JSON.parse(localStorage.getItem('signin_user'))
          let user_id =  JSON.parse(localStorage.getItem('signin_user')).id
          let data = { user_id: Number.parseInt(user_id)}
          fetch(URLprefix + 'api/user/id/messages',{
                    body: JSON.stringify(data), 
                    headers: {
                      'content-type': 'application/json'
                    },
                    method: 'POST',
                    mode: 'cors'
                }).then(response => response.json())
                .then(json => {
                    this.messages_count = json.messages_count
                })
                .catch((e) => {
                  console.log(e)
                })

            let u_id = JSON.parse(localStorage.getItem('signin_user')).id
            let data2 = { 
                user_id:  Number.parseInt(u_id)
            }
            fetch(URLprefix + 'api/community_names', {
                    body: JSON.stringify(data2), 
                    headers: {
                        'content-type': 'application/json'
                    },
                    method: 'POST',
                    mode: 'cors'
            }).then(response => response.json())
            .then(json => {
                    this.community_names = json.community_name_tags
                    console.log(this.community_names)
            })
            .catch((e) => {
                console.log(e)
            })
      }else{
        return
      }
  },
  methods: {
    /* 点击按钮，下拉菜单在 显示/隐藏 之间切换 */
    myFunction() {
        document.getElementById("myDropdown").classList.toggle("show");
    },
    logout() {
      if (localStorage.getItem('token')){
          localStorage.removeItem('token')
          localStorage.removeItem('signin_user')
          this.$router.push('/a/access')
      }else{
        return
      }
    }
  }
}
</script>

<style lang="css" scoped>
#mnav {
    background-color: #ffffff;
    position: fixed;
    width: 100%;
    z-index: 1111;
    box-shadow: 0 1px 2px 0 rgba(60, 64, 67, .3), 0 2px 6px 2px rgba(60, 64, 67, .15);
}
#lnav #left li {
    display: inline-block;
}
#lnav #dlnav  .arrow {
          font-size: 0;
          line-height: 0;
          border-width: 8px;
          border-color: #888;
          border-bottom-width: 0;
          border-style: dashed;
          /* border-top-style: solid; */
          border-left-color: transparent;
          border-right-color: transparent;
    }
    #lnav  #dlnav  .dropdown {
            margin-bottom: -0.5rem;
            line-height: 1.6rem;;
            overflow: hidden;
    }
    #lnav  #dlnav  .dropdown #dropbox {
          /* border-radius: 8%; */
          padding:  0 0.2rem;
          /* border:  1px solid rgb(198, 199, 199); */
          /* box-shadow: 1px 8px 16px 1px rgba(0,0,0,0.2); */
    }
    #lnav #dlnav  .dropbtn a {
            text-align: center;
            text-decoration: none;
    }
    #lnav #dlnav   .dropdown .dropbtn {
            cursor: pointer;
            border: none;
            outline: none;
            background-color: #f9f9f9;
            /* font-weight: bold; */
            font-size: 1rem;
        }

    #lnav  #dlnav  .dropdown-content {
            display: none;
            position: absolute;
            padding:  0.5rem 0;
            background-color: #f9f9f9;
            box-shadow: 1px 8px 16px 1px rgba(0,0,0,0.2);
            z-index: 1;
        }

    #lnav #dlnav .dropdown-content a {
            line-height: 1.5rem;
            padding:  0 0.5rem;
            display: block;
            text-align: left;
        }

    #lnav #dlnav .dropdown-content a:hover {
            background-color: #ddd;
    }

    #lnav #dlnav .show {
            display: block;
    }

#rnav #mnumber {
    padding: 0.1rem 0.3rem;
    color: red;
    background-color:yellow;
}
@media only screen and (max-width: 767px) {
    #mnav {
        margin: 0 auto;
        line-height: 50px;
    }
    #lnav {
        display: flex;
    }
    #lnav #left {
        flex-grow: 1;
    }
    #lnav #left #front img {
        width: 4.4rem;
        height: 2rem;
        vertical-align: middle;
        margin-left: 0.5rem;
    }
    #rnav {
        height: 0;
        display: block;
        overflow: hidden;
    }
    #rnav li, #rnav #mlnav {
        line-height: 40px;
    }
    #rnav #mnumber {
        padding-left: 1.1rem;
    }
    #rnav li a, #rnav #mlnav a {
      display:block;
      border-top: 1px solid grey;
      padding-left: 1.1rem;
    }
    #lnav label img {
        vertical-align:middle;
        width: 1.8rem;
        height: 1.8rem;
        margin-right: 0.5rem;
        padding: 0.3rem;
        /* border: 1px solid #dfdcdc; */
    }
    #lnav #left li {
        margin-left: 1rem;
    }
}
@media only screen and (min-width: 768px) and (max-width: 999px) {
    #menu,#mlnav {
      display: none;
    }
    #mnav {
      margin: 0 auto;
      display: flex;
      line-height: 50px;
    }
    #mnav #lnav {
       flex-grow:1;
    }
    #lnav {
        margin-left: 5%;
    }
    #rnav { 
        margin-right: 5%;
    }
    #lnav #left #front img {
        width: 4.4rem;
        height: 2rem;
        vertical-align: middle;
        margin-bottom: 0.3rem;
    }
    #lnav #left li {
        margin-left: 1.5rem;
    }
    #rnav li {
      display: inline-block;
    }
    #rnav li a {
      margin-left: 1.5rem;
    }
}
@media only screen and (min-width: 1000px) and (max-width: 1399px)  {
    #menu, #mlnav {
      display: none;
    }
    #mnav {
      margin: 0 auto;
      display: flex;
      line-height: 50px;
    }
    #mnav #lnav {
       flex-grow:1;
    }
    #lnav {
        margin-left: 5%;
    }
    #rnav { 
        margin-right: 5%;
    }
    #lnav #left #front img {
        width: 4.4rem;
        height: 2rem;
        vertical-align: middle;
        margin-bottom: 0.3rem;
    }
    #lnav #left li {
        margin-left: 1.5rem;
    }
    #rnav li {
      display: inline-block;
    }
    #rnav li a {
      margin-left: 1.5rem;
    }
}
@media only screen and (min-width: 1400px) {
    #menu,#mlnav {
      display: none;
    }
    #mnav {
      margin: 0 auto;
      display: flex;
      line-height: 50px;
    }
    #mnav #lnav {
       flex-grow:1;
    }
    #lnav {
        margin-left: 10%;
    }
    #lnav #left #front img {
        width: 4.4rem;
        height: 2rem;
        vertical-align: middle;
        margin-bottom: 0.3rem;
    }
    /* #lnav #left #name {
        color: indigo;
        font-size: 1.7rem;
        font-weight: bold;
        margin-left: 1rem;
    } */
    #lnav #left li {
        margin-left: 1.5rem   ;
        display: inline-block;
    }
    #lnav #left #tail {
        vertical-align: middle;
    }
    #rnav { 
        margin-right: 10%;
    }
    #rnav li {
        display: inline-block;
    }
    #rnav li a {
        margin-left: 1.5rem;
    }
}
</style>