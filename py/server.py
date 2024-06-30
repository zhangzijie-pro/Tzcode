from flask import Flask, request, jsonify
from flask_cors import CORS
import json
import os

app = Flask(__name__)
CORS(app)

workspace_path = os.path.join(os.path.dirname(__file__), '../space/workspace.json')
key_path = os.path.join(os.path.dirname(__file__), '../space/key.json')
space_color_path = os.path.join(os.path.dirname(__file__), '../space/space-color.json')

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
    file_path = {
        'workspace': workspace_path,
        'key': key_path,
        'space-color': space_color_path
    }.get(path)
    if file_path:
        data = read_json(file_path)
        return jsonify(data)
    else:
        return jsonify({"error": "Invalid path"}), 400

@app.route('/workspace', methods=['POST'])
def add_workspace():
    user_input = request.json.get('input')
    data = read_json(workspace_path)
    data['workspace'].append(user_input)
    write_json(workspace_path, data)
    return jsonify(data)

@app.route('/workspace', methods=['DELETE'])
def delete_workspace():
    index = int(request.json.get('index'))
    data = read_json(workspace_path)
    try:
        data['workspace'].pop(index)
    except IndexError:
        return jsonify({"error": "Invalid index"}), 400
    write_json(workspace_path, data)
    return jsonify(data)

@app.route('/key', methods=['POST'])
def add_key():
    user_input = request.json.get('input')
    data = read_json(key_path)
    data['key'].append(user_input)
    write_json(key_path, data)
    return jsonify(data)

@app.route('/key', methods=['DELETE'])
def delete_key():
    index = int(request.json.get('index'))
    data = read_json(key_path)
    try:
        data['key'].pop(index)
    except IndexError:
        return jsonify({"error": "Invalid index"}), 400
    write_json(key_path, data)
    return jsonify(data)

@app.route('/space-color', methods=['POST'])
def update_space_color():
    user_input = request.json.get('input')
    nested_keys = request.json.get('keys')
    data = read_json(space_color_path)

    d = data['workspace']['background']
    for key in nested_keys[:-1]:
        d = d[key]
    d[nested_keys[-1]] = user_input

    write_json(space_color_path, data)
    return jsonify(data)


if __name__ == '__main__':
    app.run(host='127.0.0.1', port=5000)