import paho.mqtt.client as mqtt

positions = []
velocities = []


def on_message(client, userdata, message):
    if message.topic == "CTRL/out/x":
        print(str(message.payload))
    elif message.topic == "CTRL/out/v":
        print(str(message.payload))


if __name__ == "__main__":
    print("Setting up mqtt connection")
    mqttc = mqtt.Client()
    mqttc.connect("localhost")
    print("Successfully connected to mqtt broker")
    print("Subscribing to output topics")
    mqttc.subscribe("CTRL/out/x", 2)
    mqttc.subscribe("CTRL/out/v", 2)
    print("Setting Callbacks...")
    mqttc.on_message = on_message
    mqttc.loop_forever()
    print("Looping...")
