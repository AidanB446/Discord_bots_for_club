import discord
import requests
from bs4 import BeautifulSoup as bs4


TOKEN = "TOKEN"

class MyClient(discord.Client) :

    async def on_ready(self) :
        print("bot logged in")

    async def on_message(self, msg) :

        if msg.author == self.user :
            return
        
        formatted_msg = msg.content.split(" ")
        match formatted_msg[0] :
            
            case "!!ping" :
                await msg.channel.send("pong")
            
            case "!!solve" :
                 
                if len(formatted_msg) < 2 :
                    await msg.channel.send("missing equation, command doesn't have enough arguements.")

                equa = formatted_msg[1]
                await msg.channel.send(str(eval(equa)))
            

            case "!!weather" :
                url = "https://weather.com/weather/today/l/857a297816e489427e3c6b82d20cb5cd974d1c6132de1e63e139c32f1bfca2bd" 

                r = requests.get(url=url)
                soup = bs4(r.content, 'html.parser')
                s = soup.find('div', id='todayDetails')
                await msg.channel.send(s.text)

            case _ :
                pass



intents = discord.Intents.default()
intents.message_content = True
client = MyClient(intents=intents)
client.run(TOKEN)

