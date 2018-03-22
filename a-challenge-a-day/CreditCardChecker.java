//Reveives a number and decides which credit card issuer it belongs to.
//Jonas Jore, 14.03.94


public class CreditCardChecker {

	public static String getIssuer(String cardNumber) {
		String issuer = "Unknown";
		if(cardNumber.substring(0,2).equals("34")  || cardNumber.substring(0,2).equals("37") && cardNumber.length() == 15) {
			issuer = "AMEX";
		} else if(cardNumber.substring(0,4).equals("6011") && cardNumber.length() == 16) {
			issuer = "Discover";
		} else if(cardNumber.substring(0,2).equals("51") || cardNumber.substring(0,2).equals("52") || cardNumber.substring(0,2).equals("53") || cardNumber.substring(0,2).equals("54") || cardNumber.substring(0,2).equals("55") && cardNumber.length() == 16) {
			issuer = "Mastercard";
		} else if(cardNumber.substring(0,1).equals("4") && (cardNumber.length() == 13 || cardNumber.length() == 16)) {
			issuer = "VISA";
		}
		
		return issuer;
		
	}
	
	public static void main(String[] args) {
		System.out.println(getIssuer("4111111111111111"));
	}

}
