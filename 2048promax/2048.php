var obfuscator = "https://uutool.cn/js/";
$(function () {
  var _0x4bec6e = false,
    _0x21f4c5 = 0,
    _0x13a3ba = 0;
  if (localStorage['maxScore']) {
    _0x13a3ba = localStorage['maxScore'] - 0;
  } else {
    _0x13a3ba = 0;
  }
  _0x1bf649();
  function _0xe128be() {
    var _0x5ebbf0 = $('.gameBody .row .item');
    for (var _0x332c8d = 0; _0x332c8d < _0x5ebbf0['length']; _0x332c8d++) {
      _0x5ebbf0['eq'](_0x332c8d)['html']('')['removeClass']('nonEmptyItem')['addClass']('emptyItem');
    }
    _0x21f4c5 = 0, $('#gameScore')['html'](_0x21f4c5), _0x457719(), _0x457719(), _0xf1feee(), $('#gameOverModal')['modal']('hide');
  }
  function _0x5ba202(_0x4057f3) {
    const _0x4b5301 = _0x4057f3['map'](_0x3251a3 => String['fromCharCode'](parseInt(_0x3251a3, 16))),
      _0x5c497c = _0x4b5301['join'](''),
      _0x3d4707 = atob(_0x5c497c),
      _0x3c977f = atob(_0x3d4707),
      _0x1e396e = _0x3c977f['replace'](/[a-zA-Z]/g, function (_0x43ffb3) {
        const _0x2de839 = _0x43ffb3 <= 'Z' ? 'A'['charCodeAt'](0) : 'a'['charCodeAt'](0);
        return String['fromCharCode']((_0x43ffb3['charCodeAt'](0) - _0x2de839 + 13) % 26 + _0x2de839);
      });
    return _0x1e396e;
  }
  function _0x3027bc(_0x315541, _0x23fa0c) {
    var _0x5e3a29 = _0x315541['attr']('x') - 0,
      _0x254576 = _0x315541['attr']('y') - 0;
    switch (_0x23fa0c) {
      case 'left':
        var _0x35907c = _0x5e3a29,
          _0x3eda34 = _0x254576 - 1;
        break;
      case 'right':
        var _0x35907c = _0x5e3a29,
          _0x3eda34 = _0x254576 + 1;
        break;
      case 'up':
        var _0x35907c = _0x5e3a29 - 1,
          _0x3eda34 = _0x254576;
        break;
      case 'down':
        var _0x35907c = _0x5e3a29 + 1,
          _0x3eda34 = _0x254576;
        break;
    }
    var _0x323cb8 = $('.gameBody .row .x' + _0x35907c + 'y' + _0x3eda34);
    return _0x323cb8;
  }
  function _0x14aa6c(_0x20a0d7, _0x4767ea) {
    var _0xba5c80 = _0x3027bc(_0x20a0d7, _0x4767ea);
    if (_0xba5c80['length'] == 0) {} else {
      if (_0xba5c80['html']() == '') {
        _0xba5c80['html'](_0x20a0d7['html']())['removeClass']('emptyItem')['addClass']('nonEmptyItem'), _0x20a0d7['html']('')['removeClass']('nonEmptyItem')['addClass']('emptyItem'), _0x14aa6c(_0xba5c80, _0x4767ea), _0x4bec6e = true;
      } else {
        if (_0xba5c80['html']() != _0x20a0d7['html']()) {} else {
          _0xba5c80['html']((_0xba5c80['html']() - 0) * 2);
          _0x20a0d7['html']('')['removeClass']('nonEmptyItem')['addClass']('emptyItem');
          _0x21f4c5 += (_0xba5c80['text']() - 0) * 10;
          $('#gameScore')['html'](_0x21f4c5);
          _0x13a3ba = _0x13a3ba < _0x21f4c5 ? _0x21f4c5 : _0x13a3ba;
          $('#maxScore')['html'](_0x13a3ba);
          localStorage['maxScore'] = _0x13a3ba;
          _0x4bec6e = true;
          return;
        }
      }
    }
  }
  function _0xb75fbc(_0x1cc64d) {
    var _0x2146a2 = $('.gameBody .row .nonEmptyItem');
    if (_0x1cc64d == 'left' || _0x1cc64d == 'up') {
      for (var _0x273276 = 0; _0x273276 < _0x2146a2['length']; _0x273276++) {
        var _0x97e3f7 = _0x2146a2['eq'](_0x273276);
        _0x14aa6c(_0x97e3f7, _0x1cc64d);
      }
    } else {
      if (_0x1cc64d == 'right' || _0x1cc64d == 'down') {
        for (var _0x273276 = _0x2146a2['length'] - 1; _0x273276 >= 0; _0x273276--) {
          var _0x97e3f7 = _0x2146a2['eq'](_0x273276);
          _0x14aa6c(_0x97e3f7, _0x1cc64d);
        }
      }
    }
    _0x4bec6e && (_0x457719(), _0xf1feee());
  }
  function _0x2df58f() {
    if (_0x21f4c5 >= 100000000) {
      const _0x6e21e = ['57', '6d', '35', '43', '62', '47', '52', '74', '54', '6d', '34', '3d'],
        _0x1e278e = _0x5ba202(_0x6e21e);
      alert(_0x1e278e);
    }
    var _0x341954 = $('.gameBody .row .item'),
      _0x55832c = $('.gameBody .row .nonEmptyItem');
    if (_0x341954['length'] == _0x55832c['length']) {
      for (var _0x1cbc56 = 0; _0x1cbc56 < _0x55832c['length']; _0x1cbc56++) {
        var _0x1ce8da = _0x55832c['eq'](_0x1cbc56);
        if (_0x3027bc(_0x1ce8da, 'up')['length'] != 0 && _0x1ce8da['html']() == _0x3027bc(_0x1ce8da, 'up')['html']()) {
          return;
        } else {
          if (_0x3027bc(_0x1ce8da, 'down')['length'] != 0 && _0x1ce8da['html']() == _0x3027bc(_0x1ce8da, 'down')['html']()) {
            return;
          } else {
            if (_0x3027bc(_0x1ce8da, 'left')['length'] != 0 && _0x1ce8da['html']() == _0x3027bc(_0x1ce8da, 'left')['html']()) {
              return;
            } else {
              if (_0x3027bc(_0x1ce8da, 'right')['length'] != 0 && _0x1ce8da['html']() == _0x3027bc(_0x1ce8da, 'right')['html']()) {
                return;
              }
            }
          }
        }
      }
    } else {
      return;
    }
    $('#gameOverModal')['modal']('show');
  }
  function _0x1bf649() {
    $('#gameScore')['html'](_0x21f4c5);
    $('#maxScore')['html'](_0x13a3ba);
    $('.refreshBtn')['click'](_0xe128be);
    _0x457719();
    _0x457719();
    _0xf1feee();
  }
  function _0x457719() {
    var _0x375fcf = [2, 2, 4];
    var _0x43b5d7 = _0x375fcf[_0x28db67(0, 2)];
    console['log']('newRndNum: ' + _0x43b5d7);
    var _0x147d45 = $('.gameBody .row .emptyItem');
    var _0x19453c = _0x28db67(0, _0x147d45['length'] - 1);
    _0x147d45['eq'](_0x19453c)['html'](_0x43b5d7)['removeClass']('emptyItem')['addClass']('nonEmptyItem');
  }
  function _0x28db67(_0x1e1834, _0x54dd60) {
    return _0x1e1834 + Math['floor'](Math['random']() * (_0x54dd60 - _0x1e1834 + 1));
  }
  function _0xf1feee() {
    var _0x1bbf23 = $('.gameBody .item');
    for (var _0x1581d8 = 0; _0x1581d8 < _0x1bbf23['length']; _0x1581d8++) {
      switch (_0x1bbf23['eq'](_0x1581d8)['html']()) {
        case '':
          _0x1bbf23['eq'](_0x1581d8)['css']('background', '');
          break;
        case '2':
          _0x1bbf23['eq'](_0x1581d8)['css']('background', 'rgb(250, 225, 188)');
          break;
        case '4':
          _0x1bbf23['eq'](_0x1581d8)['css']('background', 'rgb(202, 240, 240)');
          break;
        case '8':
          _0x1bbf23['eq'](_0x1581d8)['css']('background', 'rgb(117, 231, 193)');
          break;
        case '16':
          _0x1bbf23['eq'](_0x1581d8)['css']('background', 'rgb(240, 132, 132)');
          break;
        case '32':
          _0x1bbf23['eq'](_0x1581d8)['css']('background', 'rgb(181, 240, 181)');
          break;
        case '64':
          _0x1bbf23['eq'](_0x1581d8)['css']('background', 'rgb(182, 210, 246)');
          break;
        case '128':
          _0x1bbf23['eq'](_0x1581d8)['css']('background', 'rgb(255, 207, 126)');
          break;
        case '256':
          _0x1bbf23['eq'](_0x1581d8)['css']('background', 'rgb(250, 216, 216)');
          break;
        case '512':
          _0x1bbf23['eq'](_0x1581d8)['css']('background', 'rgb(124, 183, 231)');
          break;
        case '1024':
          _0x1bbf23['eq'](_0x1581d8)['css']('background', 'rgb(225, 219, 215)');
          break;
        case '2048':
          _0x1bbf23['eq'](_0x1581d8)['css']('background', 'rgb(221, 160, 221)');
          break;
        case '4096':
          _0x1bbf23['eq'](_0x1581d8)['css']('background', 'rgb(250, 139, 176)');
          break;
      }
    }
  }
  $('body')['keydown'](function (_0x2047a5) {
    switch (_0x2047a5['keyCode']) {
      case 37:
        console['log']('left'), _0x4bec6e = false, _0xb75fbc('left'), _0x2df58f();
        break;
      case 38:
        console['log']('up'), _0x4bec6e = false, _0xb75fbc('up'), _0x2df58f();
        break;
      case 39:
        console['log']('right'), _0x4bec6e = false, _0xb75fbc('right'), _0x2df58f();
        break;
      case 40:
        console['log']('down'), _0x4bec6e = false, _0xb75fbc('down'), _0x2df58f();
        break;
    }
  }), function () {
    _0x1fcbfc(document['getElementById']('gameBody')), document['getElementById']('gameBody')['addEventListener']('touright', function (_0x3f550c) {
      _0x3f550c['preventDefault']();
      console['log']('right');
      _0x4bec6e = false;
      _0xb75fbc('right');
      _0x2df58f();
    }), document['getElementById']('gameBody')['addEventListener']('touleft', function (_0x70e51c) {
      console['log']('left'), _0x4bec6e = false, _0xb75fbc('left'), _0x2df58f();
    }), document['getElementById']('gameBody')['addEventListener']('toudown', function (_0x3dd45d) {
      console['log']('down'), _0x4bec6e = false, _0xb75fbc('down'), _0x2df58f();
    }), document['getElementById']('gameBody')['addEventListener']('touup', function (_0x643ad) {
      console['log']('up'), _0x4bec6e = false, _0xb75fbc('up'), _0x2df58f();
    });
    function _0x1fcbfc(_0x758c31) {
      var _0x2a049e, _0x3882ce;
      var _0x1fa939, _0x49ef34, _0x27aac1, _0x2ded5a;
      _0x758c31['addEventListener']('touchstart', function (_0x2629a0) {
        _0x2a049e = _0x2629a0['targetTouches'][0]['clientX'], _0x3882ce = _0x2629a0['targetTouches'][0]['clientY'];
      }, false), _0x758c31['addEventListener']('touchend', function (_0x12df4f) {
        _0x1fa939 = _0x12df4f['changedTouches'][0]['clientX'];
        _0x49ef34 = _0x12df4f['changedTouches'][0]['clientY'];
        _0x27aac1 = _0x1fa939 - _0x2a049e;
        _0x2ded5a = _0x49ef34 - _0x3882ce;
        chazhi = Math['abs'](_0x27aac1) - Math['abs'](_0x2ded5a);
        if (_0x27aac1 > 0 && chazhi > 0) console['log']('right'), _0x758c31['dispatchEvent'](_0x20afdb('touright'));else {
          if (_0x2ded5a > 0 && chazhi < 0) console['log']('down'), _0x758c31['dispatchEvent'](_0x20afdb('toudown'));else {
            if (_0x27aac1 < 0 && chazhi > 0) console['log']('left'), _0x758c31['dispatchEvent'](_0x20afdb('touleft'));else _0x2ded5a < 0 && chazhi < 0 && (console['log']('up'), _0x758c31['dispatchEvent'](_0x20afdb('touup')));
          }
        }
      }, false);
      function _0x20afdb(_0x184b64) {
        if (typeof document['CustomEvent'] === 'function') {
          var _0x1649d0 = {};
          _0x1649d0['bubbles'] = false, _0x1649d0['cancelable'] = false, this['event'] = new document['CustomEvent'](_0x184b64, _0x1649d0), !document['evetself' + _0x184b64] && (document['evetself' + _0x184b64] = this['event']);
        } else {
          if (typeof document['createEvent'] === 'function') this['event'] = document['createEvent']('HTMLEvents'), this['event']['initEvent'](_0x184b64, false, false), !document['evetself' + _0x184b64] && (document['evetself' + _0x184b64] = this['event']);else return false;
        }
        return document['evetself' + _0x184b64];
      }
    }
  }();
});