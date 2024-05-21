import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import groovy.json.JsonOutput as JsonOutput

WebUI.callTestCase(findTestCase('CT/TC2_Creatcart'), [:], FailureHandling.STOP_ON_FAILURE)

//Get a  free product

CartDiscountResponse = WS.sendRequest(findTestObject('Postman/Project/Carts/Update actions/AddDiscountCode', [('host') : GlobalVariable.host
            , ('project_key') : GlobalVariable.project_key, ('cart_id') : GlobalVariable.cart_id, ('cart_version') : GlobalVariable.cart_version]))

def responseText = CartDiscountResponse.getResponseText()
println(JsonOutput.println(responseText))

//Carttotal after applying discount

CarttotalPrice = WS.getElementPropertyValue(CartDiscountResponse, 'totalPrice.centAmount')

println(('TotalPrice after appliying discount' + '') + CarttotalPrice)

if (CarttotalPrice==GlobalVariable.Cart_total) //without any other discount gift item got added
{
	println("No change in total price") 
}	
else {
	println("Cart total"+""+CarttotalPrice) //if any other discount got applied
}

Totallineitem=WS.getElementPropertyValue(CartDiscountResponse , 'totalLineItemQuantity')
println(Totallineitem)

//Giftlineitem//
if (Totallineitem==GlobalVariable.total_LineItem_Quantity + 1)
{
	println("Giftitem got added")
}
else
{
	println("Giftitem did not added")
}

//Carttotal discount


if (CarttotalPrice == (0.5 * GlobalVariable.Cart_total)) {
	println('50% Discount got applied to cart total')
} else {
	println('50% discount didnt get applied')
}


//discountamount = WS.getElementPropertyValue(CartDiscountResponse, 'discountOnTotalPrice.discountedAmount.centAmount')

//println(('Discounted amount' + '') + discountamount)