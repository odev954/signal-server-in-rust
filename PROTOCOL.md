# The Chat Application Protocol
Here you will find all the information about the communication protocol that is used by the server.

## The Chat Application
This project is working with a pre-made chat application, based on signal's one earliest builds.  
The Application looks like this:  

![image](https://user-images.githubusercontent.com/55921680/118361864-dac48980-b595-11eb-934e-0ab82d3a298c.png)

In the left side is the list of the users that are currently connected to the server. The panel in the right is the chat viewer, which contains all the messages and chat history. Users can send messages to other users that are currently connected to the server, but not to offline users (users can still view the chat history of an offline users). The application window updates itself every 200 milliseconds. Users can logout of the application by closing the window. 

To login to the application, you just type the username in the textbox and press on the 'Login' button, like so:

![image](https://user-images.githubusercontent.com/55921680/118362316-ba95ca00-b597-11eb-8454-50443370b564.png)

**Importent Note:** There are some bugs with the application, such as users cannot view messages they sent earlier in last sessions. So be aware that there are possibilities for bugs with this application (originally, I didn't programmed this application, it was provided to me by my former instructors).

## The Protocol

### File Chat Format:
A message record is formatted like so:

![image](https://user-images.githubusercontent.com/55921680/118362381-fcbf0b80-b597-11eb-8b6b-0990552ff397.png)

*<author_username>* : sender username  
*<message_data>* : message content

Those records has to be appended into the chat file.

### Protocol Specs

critiria | property
---------|---------
Port | 8826
Stateless/Statefull | Statefull
Textual/Binary | Textual

### Messages
#### Login Message:
message code (200) | length of username | username
-- | -- | -- 
3 bytes | 2 bytes | len(username)

#### Client Update Message:
message code (204) | length of partner username | partner username | message length | message contant
-- | -- | -- | -- | -- 
3 bytes | 2 bytes | len(partner username) | 5 bytes | len(message contant)

#### Server Update Message:
message code (101) | length of chat | chat contant | length of partner username | partner username | length of all connected user names | all connected user names
-- | -- | -- | -- | -- | -- | -- 
3 bytes | 5 bytes | len(chat contant) | 2 bytes | len(partner username) | 5 bytes | len(all connected user names)

Have fun! 🎉
