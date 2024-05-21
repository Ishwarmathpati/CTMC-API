<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>SetShippingMethod</name>
   <tag></tag>
   <elementGuidId>b51ee6bc-e207-4982-8cee-b7b83dba5173</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>vSsOoI70GdDiZuOAooNeqPmTEj7De97H</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;version\&quot;: ${GlobalVariable.cart_version},\n    \&quot;actions\&quot;: [\n        {\n            \&quot;action\&quot; : \&quot;setShippingMethod\&quot;,\n            \&quot;shippingMethod\&quot; : {\n              \&quot;id\&quot; : \&quot;51783526-fb2f-40ad-8cf3-351c3b4ab37a\&quot;,\n              \&quot;typeId\&quot; : \&quot;shipping-method\&quot;\n            }\n          }\n    ]\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>0db7b0a6-867a-49b3-b35b-260e2285253b</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${GlobalVariable.ctp_access_token}</value>
      <webElementGuid>df331419-caf2-4b15-973f-964cdc8c268b</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.host}/${GlobalVariable.project_key}/carts/${GlobalVariable.cart_id}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.host</defaultValue>
      <description></description>
      <id>1eaa03b4-cdf9-439a-ad06-28da6e77d946</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.project_key</defaultValue>
      <description></description>
      <id>7a5d3565-5eba-4b0c-aae4-09614cfe86c4</id>
      <masked>false</masked>
      <name>project_key</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cart_id</defaultValue>
      <description></description>
      <id>67ffb3ec-bb66-4004-be67-78dce9c01588</id>
      <masked>false</masked>
      <name>cart_id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cart_version</defaultValue>
      <description></description>
      <id>a5880d12-dcc3-4641-883a-5150d41bfe62</id>
      <masked>false</masked>
      <name>cart_version</name>
   </variables>
   <variables>
      <defaultValue>'51783526-fb2f-40ad-8cf3-351c3b4ab37a'</defaultValue>
      <description></description>
      <id>96f76e48-4ed8-4941-b5f1-ba4d73704ab5</id>
      <masked>false</masked>
      <name>shipping-method-id</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
WS.verifyElementPropertyValue(response, 'totalPrice.centAmount', &quot;MXN&quot;)
WS.verifyElementPropertyValue(response, 'shippingInfo.shippingRate.price.fractionDigits', 10000)
WS.verifyElementPropertyValue(response, 'shippingInfo.shippingRate.price.centAmount', &quot;MXN&quot;)
WS.verifyElementPropertyValue(response, 'totalPrice.fractionDigits', 10000)
WS.verifyElementPropertyValue(response, 'totalPrice.centAmount', &quot;MXN&quot;)
WS.verifyElementPropertyValue(response, 'shippingInfo.taxedPrice.totalNet.type', &quot;51783526-fb2f-40ad-8cf3-351c3b4ab37a&quot;)
WS.verifyElementPropertyValue(response, 'shippingInfo.taxedPrice.totalNet.type', &quot;51783526-fb2f-40ad-8cf3-351c3b4ab37a&quot;)
WS.verifyElementPropertyValue(response, 'shippingInfo.taxedPrice.totalNet.type', &quot;51783526-fb2f-40ad-8cf3-351c3b4ab37a&quot;)
WS.verifyElementPropertyValue(response, 'shippingInfo.shippingMethod.id', '')
WS.verifyElementPropertyValue(response, 'shippingInfo.taxedPrice.totalNet.type', &quot;51783526-fb2f-40ad-8cf3-351c3b4ab37a&quot;)
WS.verifyElementPropertyValue(response, 'shippingInfo.price', &quot;Standard Delivery&quot;)
WS.verifyElementPropertyValue(response, 'shippingInfo.shippingMethodName', '')
WS.verifyElementPropertyValue(response, 'shippingInfo.shippingMethodName', '')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
