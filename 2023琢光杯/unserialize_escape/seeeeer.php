<?php
class sing {
	public $x;
	function __construct($xx) {
		$this->x = $xx;
	}
	function __destruct() {
		eval($this->x);
        echo'win!';
	}	
}
// Thinking how to use sing?
class jump {
	public $root;
	public $pwd;
	function __construct($root, $pwd) {
		$this->root = $root;
		$this->pwd = $pwd;
	}
		function __destruct() {
		unset($this->root);
		unset($this->pwd);
	}
	function __toString() {
			echo "hehe!";
	}
}
function sw($str){
    return str_replace('ikun', 'kk', $str);
}
if (!isset($_GET['A'])){ 
    highlight_file(__FILE__); 
} 
else{
    $a=$_GET['A'];
    $b=$_GET['B'];
    $rap=new jump($a,$b);
    $baskerball=serialize($rap);
    $success=unserialize(sw($baskerball));
    echo $success;

} 
?>
