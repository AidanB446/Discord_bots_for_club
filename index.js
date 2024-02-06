import { Client, GatewayIntentBits } from 'discord.js';
import { REST, Routes } from 'discord.js';



const TOKEN = "TOKEN"; 
const CLIENT_ID = "CLIENT_ID";


// list the commands for autofill
// presented in json list format
const commands = [
	{
		name : 'ping',
		description : 'Replies with Pong!',
  	},
	{
		name : 'weather',
		description : 'tells you the weather in largo florida'
	},
	{
		name : 'solve',
		description : 'solves arithmetic operations Ex: /solve 7+2'
	},
];


const rest = new REST({ version: '10' }).setToken(TOKEN);

// trys to setup app autofill commands
try {
	console.log('Started refreshing application (/) commands.');

	await rest.put(Routes.applicationCommands(CLIENT_ID), { body: commands });
	
	console.log('Successfully reloaded application (/) commands.');

} catch (error) {
	console.error(error);
}


const client = new Client({ intents: [GatewayIntentBits.Guilds] });


client.on('ready', () => { // runs when bot starts
	console.log("ready");	
	// for debugging

});

// runs when user types message that starts with /
client.on('interactionCreate', async interaction => {

	if (!interaction.isChatInputCommand()) return;
	// we are only programming for chat messages	

	let user_msg = interaction.commandName;		

	switch (user_msg) {
	
		case "ping" :
			await interaction.reply("pong");
			break;
		
		case "solve" :
			await interaction.reply("NO, you got this bruh u can solve it");
			break;

		default :
			await interaction.reply("unrecognized command");
	}


});


client.login(TOKEN); // login and start bot
console.log("started");

