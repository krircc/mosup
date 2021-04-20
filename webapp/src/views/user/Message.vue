<template>
    <div id="usermessage">
      <div id="head">
        <div id="box">
            <img :src= hourse_user.avatar />
            <div id="word">Explore-Interactive</div>
        </div>
      </div>
      <div id="title">
          <ul>
              <li><a :href="'/a/user/' + $route.params.id" >主题</a></li>
              <li><a :href="'/a/user/' + $route.params.id + '/comment'" >评论</a></li>
              <li><a :href="'/a/user/' + $route.params.id + '/save'" >收藏</a></li>
              <li v-if="(messages_count != 0)&&(this.$route.params.id == signin_user.id)"><a :href="'/a/user/' + $route.params.id + '/message'" id="message-title">消息</a>&emsp;<button id="readall" @click="readall">全部阅读</button></li>
              <li v-else-if="(messages_count == 0)&&(this.$route.params.id == signin_user.id)"><a :href="'/a/user/' + $route.params.id + '/message'" id="message-title">消息</a></li>
              <li v-else><a :href="'/a/user/' + $route.params.id + '/message'" id="message-title"></a></li>
          </ul>
      </div>
      <main>
        <div id="container">
            <div id="center">
                <div id="items" v-for="(message, index) in messages" :key="index">
                            <div id="item">
                                <div id="infos">
                                    <span id="info"><a :href="'/a/user/' + message.from_user_id">用户{{ message.from_user_id }}</a></span>&emsp;
                                    <span id="info"><a :href="'/a/community' + '/theme/' + message.theme_id">在主题{{ message.theme_id }}回复你</a></span>&emsp;
                                    <span id="info"> {{ message.created_at }} </span>&emsp;
                                    <span v-if="message.message_status == 0" id="messagenew">新</span>
                                </div> 
                                <div id="item-title">
                                  <a :href="'/a/community' + '/theme/' + message.theme_id + '#comment'" title="theme.title"> {{ message.content }} </a>
                                </div>
                            </div>
                      </div>
            </div>
            <gotop></gotop>
        </div>
      </main>
    </div>
</template>

<script>
/* eslint-disable */
import URLprefix from '../../config'
import auth from '../../utils/auth.vue'
import Gotop from '../../components/gotop/Gotop.vue'
import Mnav from '../../components/nav/Mnav.vue'
export default {
  name: 'usermessage',
  components: {
    "gotop": Gotop,
    "mnav": Mnav
  },
  data: function() {
    return {
      theme_list: '',
      messages: '',
      messages_count: '',
      signin_user: '',
      hourse_user: '',
      messages_unread_ids: ''
    }
  },
  mounted: function() {
        if (JSON.parse(localStorage.getItem('signin_user'))) {
            this.signin_user = JSON.parse(localStorage.getItem('signin_user'))
        }
        let data = { user_id : Number.parseInt(this.$route.params.id)}
        fetch(URLprefix + 'api/user/id/messages',{
                  body: JSON.stringify(data), 
                  headers: {
                    'content-type': 'application/json'
                  },
                  method: 'POST',
                  mode: 'cors'
              }).then(response => response.json())
              .then(json => {
                  this.messages = json.messages.reverse()
                  this.messages_count = json.messages_count
                  this.messages_unread_ids = json.messages_unread_ids
                  console.log(this.messages_unread_ids)
                  console.log(this.messages_count)
              })
              .catch((e) => {
                console.log(e)
              })

        fetch(URLprefix + 'api/user_id',{
                    body: JSON.stringify(data), 
                    headers: {
                        'content-type': 'application/json'
                    },
                    method: 'POST',
                    mode: 'cors'
                }).then(response => response.json())
                .then(json => {
                    json.hourse_user.created_at = json.hourse_user.created_at.slice(0,10)
                    if (json.hourse_user.avatar == "") {
                        json.hourse_user.avatar = "https://www.gravatar.com/avatar/1"
                    }
                    this.hourse_user =  json.hourse_user
                }).catch((e) => {
                    console.log(e)
                })
  },
  methods: {
    readall() {
              let data = { 
                  user_id : Number.parseInt(this.$route.params.id),
                  messages_unread_ids: this.messages_unread_ids
              }
              fetch(URLprefix + 'api/user/id/messages/readall',{
                  body: JSON.stringify(data), 
                  headers: {
                    'content-type': 'application/json'
                  },
                  method: 'POST',
                  mode: 'cors'
              }).then(response => response.json())
              .then(json => {
                 window.location.reload ( true );
              })
              .catch((e) => {
                console.log(e)
              })
    }
  }
}
</script>

<style scoped>
#head {
    background-color: #f1a3d6;
}
#box {
    display: flex;
}
#title {
    line-height: 3.3rem;
    background-color: #faeaf5;
}
#title #message-title {
    padding-bottom: 0.2rem;
    border-bottom: 2px solid #a506a5;
}
#container a{
    color: #0541af;
}
#center #items #item {
    padding: 1.2vh 0.5rem;
    border-bottom: 1px solid #f3e1f8;
}
#center #items #item-title {
    margin-top: 1vh;
}
#title li #readall {
        border-radius: 50%;
        padding: 0.1vh 0.2vw;
        border: 1px solid #aac;
        background-color: #f0c7fc;
}
#center #items #infos #messagenew {
    padding: 0.2vh 0.2vw;
    font-size: 0.8rem;
    border-radius: 50%;
    color: yellow;
    background-color: fuchsia;
}
#center #items #infos {
    font-size: 0.85rem;
}
@media only screen and (max-width: 767px) {
    #head {
        padding-top: 3.2rem;
    }
    img {
        margin: 0.5rem;
        width: 3rem;
        height: 3rem;
        border-radius: 50%;
    }
    #word {
        padding: 1.2rem 0.6rem;
        font-size: 1.5rem;
        font-weight: bold;
        color: green;
    }
    #title ul {
        margin-left: 2vw;
    }
    #title ul li {
        display: inline-block;
        padding-right: 1.5rem;
    }
    main{
        margin: 0 auto 1rem;
        width: 97%;
    }
}
@media only screen and (min-width: 768px) and (max-width: 999px) {
    #head {
        padding-top: 2.5rem;
    }
    img {
        margin: auto 0 0.8rem 8vw;
        width: 4em;
        height: 4rem;
        border-radius: 50%;
    }
    #word {
        padding: 2rem;
        font-size: 2rem;
        font-weight: bold;
        color: green;
    }
    #title ul {
        margin-left: 6.5vw;
    }
    #title ul li {
        display: inline-block;
        padding-left: 2rem;
    }
    main {
        margin: 1rem auto;
        width: 80%;
    }
}
@media only screen and (min-width: 1000px) {
    #head {
        padding-top: 4rem;
    }
    img {
        width: 5rem;
        height: 5rem;
        border-radius: 50%;
    }
    #word {
        padding: 1.6rem;
        font-size: 2rem;
        font-weight: bold;
        color: green;
    }
    #head #box, #title ul {
        margin: 0 auto;
        width: 1040px;
    }
    #title ul li{
        display: inline-block;
        padding-left: 2rem;
    }
    main {
        margin: 1rem auto;
        width: 1040px;
    }
    #container {
        margin: 0 auto;
        width: 1000px;
        display: flex;
        flex-flow: row;
    }
    #container #center {
        width: 80%;
        
    }
}
</style>