<template>
  <div id="community-tag">
      <div id="show">
          <div id="logo">
            <span><img :src="the_community.community_img" /></span>
            <span id="community-name">{{the_community.community_name_cn}}
                <span v-if="(joinor == false) && signin_user.id" id="join" @click="join">加入</span>
                <span v-else id="joined">已加入</span>
                <span id="num">总 {{join_num}}</span>
            </span>
          </div>
      </div>
      <div id="header">
          <ul>
            <li><a :href="'/a/community/' + community_name">最新</a></li>
            <!-- <li id="item"><a :href="'/a/community/' + community_name + '/' + 'hot'">最热</a></li> -->
            <span v-for="(community_name_ta, index) in community_name_tags" :key="index">
                  <li id="item" v-if="tag_name != 'Announce'">
                      <a :href="'/a/community/' + community_name + '/' + community_name_ta.tag_name" >{{ community_name_ta.tag_name_cn }}</a>
                  </li>
            </span>
          </ul>
      </div>
      <div id="line"></div>
      <main>
        <div id="container">
          <div id="center">
              <div id="content">
                    <div id="items" v-for="(theme, index) in community_theme_list" :key="index">
                        <div id="theme">
                            <di id="avatar">
                                <a :href="'/a/user/' + theme.user_id"><img :src= theme.user_avatar ></a>
                            </di>
                            <div id="mobile">
                                    <div id="title">
                                        <span id="community-tag-name"><a :href="'/a/community/'  + community_name + '/'  + theme.community_tag_name">{{theme.community_tag_name_cn}}</a></span>
                                        <span id="name"><a :href="'/a/'+ community_name + '/theme/' + theme.id" title="theme.title">{{theme.title}}</a></span>
                                    </div> 
                                    <div id="detail">
                                        <div class="info" id="comment"><a :href="'/a/'+ community_name + '/theme/' + theme.id">{{theme.comment_count}}</a></div>
                                        <div class="info" id="view">{{theme.view_count}}</div>
                                        <div class="info" id="time">{{theme.rtime}}</div>
                                    </div> 
                            </div>
                        </div>
                    </div>
              </div>
          </div>
          <div id="aside">
              <side></side>
          </div>
        </div>
      </main>
  </div>
</template>

<script>
/* eslint-disable */
import URLprefix from '../../config'
import Side from '../../components/side/Side.vue'
export default {
  name: 'community-tag',
  components: {
    "side": Side
  },
  data: function() {
    return {
      community_theme_list: '',
      signin_user: '',
      community_name:'',
      the_community: '',
      community_name_tags: '',
      joinor: ''
    }
  },
  mounted: function() {
        this.joinor = false   
        this.community_name = this.$route.params.name
        let data = { 
            community_name : this.$route.params.name,
            community_name_tag: this.$route.params.tag 
        }
        fetch(URLprefix + 'api/community/'+ this.$route.params.name,{
                body: JSON.stringify(data), 
                headers: {
                    'content-type': 'application/json'
                },
                method: 'POST',
                mode: 'cors'
        }).then(response => response.json()).then(json => {
                json.community_theme_list.map((item) => {
                    if (item.user_avatar == "") {
                    item.user_avatar = "https://www.gravatar.com/avatar/1"
                    }
                })
                this.community_theme_list = json.community_theme_list
                this.community_name_tags = json.community_tag_names
        }).catch((e) => { console.log(e) })
        
        if (JSON.parse(localStorage.getItem('signin_user'))) {
            this.signin_user = JSON.parse(localStorage.getItem('signin_user'))
            let uid = JSON.parse(localStorage.getItem('signin_user')).id
            
            let infodata = { 
                user_id:  Number.parseInt(uid),
                community_name : this.$route.params.name
            }
            console.log(infodata)
            fetch(URLprefix + 'api/community_info',{
                    body: JSON.stringify(infodata), 
                    headers: {
                        'content-type': 'application/json'
                    },
                    method: 'POST',
                    mode: 'cors'
            }).then(response => response.json()).then(json => {
                    this.join_num = json.number
                    this.joinor = json.joinor
                    this.the_community = json.the_community
            }).catch((e) => { console.log(e) })
        }else{
            let infodata = { 
                user_id:  Number.parseInt(0),
                community_name : this.$route.params.name
            }
            console.log(infodata)
            fetch(URLprefix + 'api/community_info',{
                    body: JSON.stringify(infodata), 
                    headers: {
                        'content-type': 'application/json'
                    },
                    method: 'POST',
                    mode: 'cors'
            }).then(response => response.json()).then(json => {
                    this.join_num = json.number
                    this.joinor = json.joinor
                    this.the_community = json.the_community
            }).catch((e) => { console.log(e) })
        }
  },
  methods: {
    join() {
        if (localStorage.getItem('signin_user')){
            let join = document.getElementById("join")
            join.style.color = "#797bf8"
            join.innerHTML = "已加入"
            let user_id = JSON.parse(localStorage.getItem('signin_user')).id
            let community_name = this.$route.params.name
            let data = { 
                user_id: user_id,
                community_name: community_name
            }
            fetch(URLprefix + 'api/community_join', {
                  body: JSON.stringify(data), 
                  headers: {
                    'content-type': 'application/json'
                  },
                  method: 'POST',
                  mode: 'cors'
            }).then(response => response.json()).then(json => {
                   window.location.reload ( true )
              }).catch((e) => {
                console.log(e)
              })
        }
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
#show {
    background-color: #ecf0f0;
}
#show #join, #show #joined {
    font-size: 1.1rem;
    font-weight: bold;
    /* vertical-align: middle; */
    padding: 0.22rem 0.1rem 0.1rem;
    border: 1px solid #c9cfcf;
}
#show #join {
    background-color:#6ceff3;
}
#show #joined {
    background-color:#02ca13;
}
#community-name {
    margin-left: 0.5rem;
    font-size: 1.8rem;
    font-weight: bold;
    vertical-align: middle;
}
#show #num {
        font-size: 1.3rem; 
        margin-left: 0.5rem;
}
#header {
    line-height: 26px;
    margin: 0.5rem 0.1rem;
}
#line {
    border: 1px solid #ecf0f0;
}
#container #center {
    border: 1px solid #ecf0f0;
}
#container #center #content #items #theme {
    display: flex;
    border-bottom: 1px solid #f3e1f8;
}

#center #content #title #name {
    padding-left: 0.3rem;
}
#center #theme #title #name a:visited {
    color: gray;
}
#center #theme #community-tag-name {
    border-radius: 10%;
    color: #1362f5;
    font-size: 0.8rem;
    background-color: #dddddd;
    padding: 0.2rem;
}
@media only screen and (max-width: 767px) {
    #show {
        padding-top: 3.1rem;
    }
    #show img {
        vertical-align: middle;
        margin: 0.3rem;
        width: 2.5rem;
        height: 2.5rem;
        /* border-radius: 50%; */
    }
    #show #community-name {
        font-size: 1.5rem;
    }
    #show #num {
        font-size: 1.2rem; 
        margin-left: 0.5rem;
    }
    #header ul {
        margin-left: 0.6rem;
    }
    #header li {
        display: inline-block;
    }
    #header #item{
        padding-left: 1rem;
    }
    main{
        margin: 0.4rem auto;
        width: 97%;
    }
    main #center {
       margin-bottom: 1rem;
    }
    #container #center #theme #avatar img {
        width: 2.3rem;
        height: 2.3rem;
        margin: 0.2rem;
        vertical-align: middle;
        border-radius: 50%;
    }
    #center #items #theme #title {
        padding: 0.4rem 0.3rem 0.1rem 0.1rem;
        flex: 1;
    }
    #center #items #theme #detail {
        display: flex;
        color:gray;
        margin-bottom: 0.2rem;
        font-size: 0.88rem;
    }
    #center #items #detail #comment,#center #items #detail #view,#center #items #detail #time {
        margin:0 1rem 0 0.2rem;
        padding-top: 0.3rem;
    }
}
@media only screen and (min-width: 768px) and (max-width: 999px) {
    #show {
        padding-top: 3.2rem;
    }
    #show #logo {
        margin-left: 7%;
    }
    #show img {
        vertical-align: middle;
        margin: 0.5rem;
        width: 4rem;
        height: 4rem;
        /* border-radius: 50%; */
    }
    #header ul {
        margin-left: 6%;
    }
    #header ul li {
        display: inline-block;
        padding-left: 2rem;
    }
    main {
        margin: 0.5rem auto 1rem;
        width: 80%;
    }
    main #center {
       margin-bottom: 1rem;
    }
    #container #center #theme #avatar img {
        width: 2.4rem;
        height: 2.4rem;
        margin-top: 0.2rem;
        border-radius: 50%;
    }
    #center #content #items #theme #mobile {
        display: flex;
        flex: 1;
    }
    #center #items #theme #title {
        width: 77%;
        padding: 0.6rem 0.2rem;
    }
    #center #items #detail {
        flex: 1;
        display: flex;
        padding-top: 0.8rem;
        color:gray;
        font-size: 0.88rem;
    }
    #center #items #detail #comment {
        text-align: center;
        flex-basis: 2rem;
        color: #9d5cf1;
    }
    #center #items #detail #view {
        text-align: center;
        flex-basis: 3rem;
    }
    #center #items #detail #time {
        text-align: center;
        flex-grow:1;
    }
}
@media only screen and (min-width: 1000px) {
    #show {
        padding-top: 3.5rem;
    }
    #show #logo {
        margin: 0 auto;
        width: 1100px;
    }
    #show img {
        vertical-align: middle;
        margin: 1rem;
        width: 4rem;
        height: 4rem;
        /* border-radius: 50%; */
    }
    #header {
        margin: 0 auto;
        width: 1040px;
    }
    #header ul {
        margin: 0 auto;
        width: 1030px;
    }
    #header li {
        display: inline-block;
    }
    #header #item{
        padding-left: 2rem;
    }
    main {
        margin: 0 auto;
        width: 1040px
    }
    #container {
      margin-top: 0.5rem;
      display: flex;
      flex-flow: row;
    }
    #container #center {
        width: 80%;
        margin-right: 0.8rem;
    }
    #container #center #theme #avatar img {
        width: 2.4rem;
        height: 2.4rem;
        margin-top: 0.2rem;
        border-radius: 50%;
    }
    #container #center #content #theme #mobile {
        display: flex;
        flex: 1;
    }
    #center #items #theme #title {
        width: 77%;
        padding: 0.5rem 0.2rem;
    }
    #center #items #detail {
        flex: 1;
        display: flex;
        padding-top: 0.8rem;
        color:gray;
        font-size: 0.88rem;
    }
    #center #items #detail #comment {
        text-align: center;
        flex-basis: 2rem;
        color: #9d5cf1;
    }
    #center #items #detail #view {
        text-align: center;
        flex-basis: 3rem;
    }
    #center #items #detail #time {
        text-align: center;
        flex-grow:1;
    }
    #container #aside {
        flex: 1;
    }
}
</style>