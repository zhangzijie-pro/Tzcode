from flask import Flask, request, jsonify
from flask_cors import CORS
import json
import os

app = Flask(__name__)
CORS(app)

# Paths to JSON files
workspace_path = os.path.join(os.path.dirname(__file__), '../space/workspace.json')
key_path = os.path.join(os.path.dirname(__file__), '../space/key.json')

def read_json(file_path):
    try:
        with open(file_path, 'r') as file:
            return json.load(file)
    except Exception as e:
        return {"error": str(e)}

def write_json(file_path, data):
    try:
        with open(file_path, 'w') as file:
            json.dump(data, file, indent=2)
    except Exception as e:
        return {"error": str(e)}

@app.route('/<path>', methods=['GET'])
def get_data(path):
    file_path = workspace_path if path == 'workspace' else key_path
    data = read_json(file_path)
    if "error" in data:
        return jsonify(data), 500
    return jsonify(data)

@app.route('/receive-message', methods=['GET'])
def receive_message():
    message = "This is a local message."
    return jsonify({"message": message})

@app.route('/<path>', methods=['POST'])
def send_input(path):
    file_path = workspace_path if path == 'workspace' else key_path
    data = request.json
    user_input = data.get('input')
    if not user_input:
        return jsonify({"error": "No input provided"}), 400

    json_data = read_json(file_path)
    if "error" in json_data:
        return jsonify(json_data), 500

    item_list = json_data.get(path, [])
    if user_input not in item_list:
        item_list.append(user_input)
        json_data[path] = item_list
        write_json(file_path, json_data)
    
    return jsonify({path: item_list})

@app.route('/<path>', methods=['DELETE'])
def delete_input(path):
    file_path = workspace_path if path == 'workspace' else key_path
    data = request.json
    index = data.get('index')
    if index is None:
        return jsonify({"error": "No index provided"}), 400

    json_data = read_json(file_path)
    if "error" in json_data:
        return jsonify(json_data), 500

    item_list = json_data.get(path, [])
    try:
        index = int(index)
        if 0 <= index < len(item_list):
            item_list.pop(index)
            json_data[path] = item_list
            write_json(file_path, json_data)
        else:
            return jsonify({"error": "Index out of range"}), 400
    except ValueError:
        return jsonify({"error": "Invalid index"}), 400
    
    return jsonify({path: item_list})

if __name__ == '__main__':
    app.run(host='127.0.0.1', port=5000)
